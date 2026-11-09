pub mod content;
pub mod credential;
pub mod lifecycle;

use std::collections::HashMap;

use axum::http::HeaderMap;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use validator::ValidationErrors;

use crate::{
    models::{
        comment::{self, Entity as Comment, Status},
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        subscriber,
    },
    services::rate_limit::{Limit, RateLimiter, client_ip},
    state::Urls,
};

pub use content::Content;
pub use credential::Pending;
pub use lifecycle::Author;

pub const MAX_BODY: u64 = 5000;
pub const MAX_NAME: u64 = 80;
const RECENT: u64 = 10;
const PER_CLIENT: Limit = Limit::per_hour(30);
const PER_ADDRESS: Limit = Limit::per_hour(5);
const PER_COMMENTER: Limit = Limit::per_hour(20);

#[derive(Clone, Copy, Debug)]
pub enum Field {
    Name,
    Email,
}

#[derive(Clone, Copy, Debug)]
pub enum Rejection {
    TooMany,
    Invalid(Field),
    Content(content::Invalid),
    Expired,
    EmailNeeded,
    EmailFailed,
}

impl From<ValidationErrors> for Rejection {
    fn from(errors: ValidationErrors) -> Self {
        let failed = errors.field_errors();
        let field = if failed.contains_key("name") { Field::Name } else { Field::Email };
        Rejection::Invalid(field)
    }
}

impl From<content::Invalid> for Rejection {
    fn from(invalid: content::Invalid) -> Self {
        Rejection::Content(invalid)
    }
}

pub fn within_limits(limits: &RateLimiter, headers: &HeaderMap, publication_id: &str, commenter: &Commenter, email: Option<&str>) -> Result<(), Rejection> {
    let checks = match commenter {
        Commenter::Member(member, _) => vec![(format!("comment:member:{}", member.user_id), PER_COMMENTER)],
        Commenter::Subscriber(subscriber) => vec![(format!("comment:subscriber:{}", subscriber.token), PER_COMMENTER)],
        Commenter::Reader => {
            let by_client = client_ip(headers).map(|ip| (format!("comment:{ip}"), PER_CLIENT));
            let by_address = email.map(|email| (format!("comment:{publication_id}:{email}"), PER_ADDRESS));
            by_client.into_iter().chain(by_address).collect()
        }
    };
    if limits.allow_all(&checks) { Ok(()) } else { Err(Rejection::TooMany) }
}

#[derive(Clone)]
pub struct Member {
    pub user_id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Login {
    Showroom,
    Link,
}

pub enum Commenter {
    Member(Member, Login),
    Subscriber(subscriber::Model),
    Reader,
}

impl Commenter {
    pub fn is_owner(&self, publication: &publication::Model) -> bool {
        matches!(self, Commenter::Member(member, _) if member.user_id == publication.owner_id)
    }
}

pub async fn members(db: &DatabaseConnection, urls: &Urls, user_ids: &[String]) -> Result<HashMap<String, Member>, DbErr> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let publications = Publication::find()
        .filter(publication::Column::OwnerId.is_in(user_ids.iter().cloned()))
        .order_by_asc(publication::Column::IsDefault)
        .order_by_desc(publication::Column::CreatedAt)
        .all(db)
        .await?;
    Ok(publications
        .into_iter()
        .map(|publication| {
            let member = Member { user_id: publication.owner_id.clone(), name: publication.name.clone(), url: Some(urls.publication(&publication.slug)) };
            (publication.owner_id, member)
        })
        .collect())
}

pub async fn is_subscribed(db: &DatabaseConnection, publication_id: &str, email: &str) -> Result<bool, DbErr> {
    let subscriber = subscriber::Entity::find()
        .filter(subscriber::Column::PublicationId.eq(publication_id))
        .filter(subscriber::Column::Email.eq(email))
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .one(db)
        .await?;
    Ok(subscriber.is_some())
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Badge {
    Author,
    Member,
}

pub struct Shown {
    pub id: String,
    pub name: String,
    pub href: Option<String>,
    pub badge: Option<Badge>,
    pub content: Content,
    pub created_at: DateTime<FixedOffset>,
    pub is_awaiting: bool,
}

pub async fn shown(db: &DatabaseConnection, urls: &Urls, publication: &publication::Model, comments: Vec<comment::Model>) -> Result<Vec<Shown>, DbErr> {
    let mut user_ids: Vec<String> = comments.iter().filter_map(|comment| comment.user_id.clone()).collect();
    user_ids.sort();
    user_ids.dedup();
    let members = members(db, urls, &user_ids).await?;

    Ok(comments
        .into_iter()
        .map(|comment| {
            let is_awaiting = lifecycle::is_awaiting(&comment);
            let fallback = comment.name.clone().unwrap_or_else(|| "Anonymous".to_string());
            let (name, href, badge) = match comment.user_id.as_deref() {
                Some(user_id) if user_id == publication.owner_id => {
                    (publication.name.clone(), Some(urls.publication(&publication.slug)), Some(Badge::Author))
                }
                Some(user_id) => match members.get(user_id) {
                    Some(member) => (member.name.clone(), member.url.clone(), Some(Badge::Member)),
                    None => (fallback, None, Some(Badge::Member)),
                },
                None => (fallback, None, None),
            };
            Shown { id: comment.id, name, href, badge, content: Content::from_stored(&comment.content), created_at: comment.created_at, is_awaiting }
        })
        .collect())
}

pub async fn for_newsletter(db: &DatabaseConnection, newsletter_id: &str, pending: &Pending) -> Result<Vec<comment::Model>, DbErr> {
    let awaiting = Condition::all()
        .add(comment::Column::Status.eq(Status::Unverified))
        .add(comment::Column::ExpiresAt.gt(Utc::now()))
        .add(comment::Column::Id.is_in(pending.ids().iter().cloned()));
    Comment::find()
        .filter(comment::Column::NewsletterId.eq(newsletter_id))
        .filter(Condition::any().add(comment::Column::Status.eq(Status::Published)).add(awaiting))
        .order_by_desc(comment::Column::CreatedAt)
        .all(db)
        .await
}

pub async fn in_newsletter(db: &DatabaseConnection, newsletter_id: &str, id: &str) -> Result<Option<comment::Model>, DbErr> {
    Comment::find_by_id(id).filter(comment::Column::NewsletterId.eq(newsletter_id)).one(db).await
}

pub async fn in_publication(db: &DatabaseConnection, publication_id: &str, id: &str) -> Result<Option<(comment::Model, newsletter::Model)>, DbErr> {
    let row = Comment::find_by_id(id)
        .find_also_related(Newsletter)
        .filter(newsletter::Column::PublicationId.eq(publication_id))
        .one(db)
        .await?;
    Ok(row.and_then(|(comment, newsletter)| Some((comment, newsletter?))))
}

pub async fn recent(db: &DatabaseConnection, publication_id: &str) -> Result<Vec<(comment::Model, newsletter::Model)>, DbErr> {
    let rows = Comment::find()
        .find_also_related(Newsletter)
        .filter(newsletter::Column::PublicationId.eq(publication_id))
        .filter(comment::Column::Status.eq(Status::Published))
        .order_by_desc(comment::Column::CreatedAt)
        .limit(RECENT)
        .all(db)
        .await?;
    Ok(rows.into_iter().filter_map(|(comment, newsletter)| Some((comment, newsletter?))).collect())
}
