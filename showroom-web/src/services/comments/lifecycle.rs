use std::time::Duration as StdDuration;

use chrono::{Duration, Utc};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    TransactionTrait, prelude::DateTimeWithTimeZone, sea_query::{Expr, Func},
};

use crate::{
    models::{
        comment::{self, Entity as Comment, Source, Status},
        newsletter::Entity as Newsletter,
        subscriber::{self, Entity as Subscriber},
        user::{self, Entity as User},
    },
    services::{comments::{Content, Member}, signing::Signer},
};

pub const UNVERIFIED_TTL: Duration = Duration::days(7);
const VERIFY: &str = "comment-verify";
const SWEEP_EVERY: StdDuration = StdDuration::from_secs(60 * 60);

pub enum Author<'a> {
    Member(&'a Member),
    Subscriber(&'a subscriber::Model),
    Claimed { email: &'a str, name: Option<&'a str> },
}

pub enum Verified {
    Published(comment::Model),
    Removed,
    Expired,
}

pub fn verify_token(signer: &Signer, comment_id: &str) -> String {
    format!("{comment_id}.{}", signer.sign(VERIFY, &[comment_id]))
}

pub fn open_verify_token<'t>(signer: &Signer, token: &'t str) -> Option<&'t str> {
    let (id, signature) = token.split_once('.')?;
    signer.verify(VERIFY, &[id], signature).then_some(id)
}

pub fn is_awaiting(comment: &comment::Model) -> bool {
    comment.status == Status::Unverified && comment.expires_at.is_some_and(|expires_at| expires_at > Utc::now())
}

pub async fn create(db: &DatabaseConnection, newsletter_id: &str, author: Author<'_>, content: &Content) -> Result<comment::Model, DbErr> {
    let now = Utc::now().fixed_offset();
    let (status, expires_at, subscriber_token, user_id, name, email) = match author {
        Author::Member(member) => (Status::Published, None, None, Some(member.user_id.as_str()), Some(member.name.as_str()), None),
        Author::Subscriber(subscriber) => {
            (Status::Published, None, Some(subscriber.token.as_str()), None, subscriber.name.as_deref(), Some(subscriber.email.as_str()))
        }
        Author::Claimed { email, name } => (Status::Unverified, Some(now + UNVERIFIED_TTL), None, None, name, Some(email)),
    };

    comment::ActiveModel {
        id: Set(nanoid!(14)),
        newsletter_id: Set(newsletter_id.to_string()),
        subscriber_token: Set(subscriber_token.map(str::to_string)),
        user_id: Set(user_id.map(str::to_string)),
        name: Set(name.map(str::to_string)),
        email: Set(email.map(str::to_string)),
        content: Set(content.to_value()),
        status: Set(status),
        source: Set(Source::Web),
        section_id: Set(None),
        created_at: Set(now),
        expires_at: Set(expires_at),
        removed_at: Set(None),
    }
    .insert(db)
    .await
}

pub async fn discard(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
    Comment::delete_many()
        .filter(comment::Column::Id.eq(id))
        .filter(comment::Column::Status.eq(Status::Unverified))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn verify(db: &DatabaseConnection, id: &str) -> Result<Option<Verified>, DbErr> {
    let txn = db.begin().await?;
    let published = Comment::update_many()
        .col_expr(comment::Column::Status, Expr::value(Status::Published))
        .col_expr(comment::Column::ExpiresAt, Expr::value(Option::<DateTimeWithTimeZone>::None))
        .filter(comment::Column::Id.eq(id))
        .filter(comment::Column::Status.eq(Status::Unverified))
        .filter(comment::Column::ExpiresAt.gt(Utc::now()))
        .exec_with_returning(&txn)
        .await?
        .into_iter()
        .next();

    let verified = match published {
        Some(comment) => Some(Verified::Published(enrol(&txn, comment).await?)),
        None => Comment::find_by_id(id).one(&txn).await?.map(|comment| match comment.status {
            Status::Published => Verified::Published(comment),
            Status::Removed => Verified::Removed,
            Status::Unverified => Verified::Expired,
        }),
    };
    txn.commit().await?;
    Ok(verified)
}

async fn enrol(txn: &impl ConnectionTrait, comment: comment::Model) -> Result<comment::Model, DbErr> {
    let (Some(email), Some(newsletter)) = (comment.email.clone(), Newsletter::find_by_id(&comment.newsletter_id).one(txn).await?) else {
        return Ok(comment);
    };
    let existing = Subscriber::find()
        .filter(subscriber::Column::PublicationId.eq(&newsletter.publication_id))
        .filter(subscriber::Column::Email.eq(&email))
        .one(txn)
        .await?;

    let subscriber = match existing {
        Some(subscriber) => {
            let name = subscriber.name.clone().or_else(|| comment.name.clone());
            let mut active: subscriber::ActiveModel = subscriber.into();
            active.is_confirmed = Set(true);
            active.name = Set(name);
            active.update(txn).await?
        }
        None => {
            subscriber::ActiveModel {
                token: Set(nanoid!(21)),
                publication_id: Set(newsletter.publication_id),
                name: Set(comment.name.clone()),
                email: Set(email),
                is_confirmed: Set(true),
                created_at: Set(Utc::now().fixed_offset()),
            }
            .insert(txn)
            .await?
        }
    };

    let account = User::find()
        .filter(Expr::expr(Func::lower(Expr::col(user::Column::Email))).eq(&subscriber.email))
        .one(txn)
        .await?;

    let mut active: comment::ActiveModel = comment.into();
    active.subscriber_token = Set(Some(subscriber.token));
    if let Some(account) = account {
        active.user_id = Set(Some(account.id));
    }
    active.update(txn).await
}

pub async fn remove(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
    Comment::update_many()
        .col_expr(comment::Column::Status, Expr::value(Status::Removed))
        .col_expr(comment::Column::RemovedAt, Expr::value(Utc::now().fixed_offset()))
        .col_expr(comment::Column::ExpiresAt, Expr::value(Option::<DateTimeWithTimeZone>::None))
        .filter(comment::Column::Id.eq(id))
        .filter(comment::Column::Status.ne(Status::Removed))
        .exec(db)
        .await?;
    Ok(())
}

pub fn spawn_sweeper(db: DatabaseConnection) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(SWEEP_EVERY);
        loop {
            interval.tick().await;
            let result = Comment::delete_many()
                .filter(comment::Column::Status.eq(Status::Unverified))
                .filter(comment::Column::ExpiresAt.lt(Utc::now()))
                .exec(&db)
                .await;
            if let Err(error) = result {
                tracing::error!(%error, "sweeping expired comments failed");
            }
        }
    })
}
