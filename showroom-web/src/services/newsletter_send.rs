use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::Duration,
};

use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
    sea_query::{Expr, LockBehavior, LockType, OnConflict, Query},
};

use crate::{
    mailer::{BATCH_SIZE, MailError, NewsletterMailing, Outcome},
    models::{
        delivery::{self, Status},
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        subscriber::{self, Entity as Subscriber},
    },
    state::AppState,
};

const MAX_ATTEMPTS: i32 = 4;
const IDLE: Duration = Duration::from_secs(2);
const PREPARE_RETRY: chrono::Duration = chrono::Duration::seconds(60);
const STALE_CLAIM: chrono::Duration = chrono::Duration::minutes(10);
const INSERT_CHUNK: usize = 500;
const RECENT: u64 = 8;

type Timestamp = DateTime<FixedOffset>;

fn now() -> Timestamp {
    Utc::now().fixed_offset()
}

pub fn address(email: &str) -> String {
    email.trim().to_lowercase()
}

fn due(at: Timestamp) -> Condition {
    Condition::any()
        .add(delivery::Column::NextAttemptAt.is_null())
        .add(delivery::Column::NextAttemptAt.lte(at))
}

fn backoff(attempts: i32) -> chrono::Duration {
    chrono::Duration::seconds(30 << (attempts - 1).clamp(0, 10))
}

pub async fn enqueue(db: &DatabaseConnection, newsletter: &newsletter::Model, publication: &publication::Model) -> Result<bool, DbErr> {
    let txn = db.begin().await?;
    let started_at = now();

    let started = Newsletter::update_many()
        .col_expr(newsletter::Column::PublishedAt, Expr::value(started_at))
        .col_expr(newsletter::Column::SendStartedAt, Expr::value(started_at))
        .filter(newsletter::Column::Id.eq(&newsletter.id))
        .filter(newsletter::Column::PublishedAt.is_null())
        .filter(newsletter::Column::SendStartedAt.is_null())
        .exec(&txn)
        .await?
        .rows_affected
        == 1;

    if !started {
        txn.rollback().await?;
        return Ok(false);
    }

    let subscribers = publication
        .find_related(Subscriber)
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .all(&txn)
        .await?;

    let mut seen = HashSet::new();
    let rows: Vec<delivery::ActiveModel> = subscribers
        .iter()
        .filter(|subscriber| seen.insert(address(&subscriber.email)))
        .map(|subscriber| delivery::ActiveModel {
            newsletter_id: Set(newsletter.id.clone()),
            email: Set(address(&subscriber.email)),
            subscriber_token: Set(subscriber.token.clone()),
            status: Set(Status::Queued),
            attempts: Set(0),
            next_attempt_at: Set(None),
            claimed_at: Set(None),
            message_id: Set(None),
            error: Set(None),
            sent_at: Set(None),
            created_at: Set(started_at),
        })
        .collect();

    for chunk in rows.chunks(INSERT_CHUNK) {
        delivery::Entity::insert_many(chunk.to_vec())
            .on_conflict(
                OnConflict::columns([delivery::Column::NewsletterId, delivery::Column::Email])
                    .do_nothing()
                    .to_owned(),
            )
            .do_nothing()
            .exec(&txn)
            .await?;
    }

    txn.commit().await?;
    Ok(true)
}

pub async fn retry_failed(db: &DatabaseConnection, newsletter_id: &str) -> Result<u64, DbErr> {
    Ok(delivery::Entity::update_many()
        .col_expr(delivery::Column::Status, Expr::value(Status::Queued))
        .col_expr(delivery::Column::Attempts, Expr::value(0))
        .col_expr(delivery::Column::NextAttemptAt, Expr::value(None::<Timestamp>))
        .col_expr(delivery::Column::Error, Expr::value(None::<String>))
        .filter(delivery::Column::NewsletterId.eq(newsletter_id))
        .filter(delivery::Column::Status.eq(Status::Failed))
        .exec(db)
        .await?
        .rows_affected)
}

pub struct Progress {
    pub queued: u64,
    pub sending: u64,
    pub sent: u64,
    pub failed: u64,
    pub unknown: u64,
    pub skipped: u64,
    pub recent: Vec<delivery::Model>,
    pub problems: Vec<delivery::Model>,
}

impl Progress {
    pub fn total(&self) -> u64 {
        self.queued + self.sending + self.sent + self.failed + self.unknown + self.skipped
    }

    pub fn done(&self) -> u64 {
        self.total() - self.queued - self.sending
    }

    pub fn active(&self) -> bool {
        self.queued + self.sending > 0
    }
}

pub async fn progress(db: &DatabaseConnection, newsletter_id: &str) -> Result<Progress, DbErr> {
    let counts: Vec<(Status, i64)> = delivery::Entity::find()
        .select_only()
        .column(delivery::Column::Status)
        .column_as(Expr::col(delivery::Column::Email).count(), "count")
        .filter(delivery::Column::NewsletterId.eq(newsletter_id))
        .group_by(delivery::Column::Status)
        .into_tuple()
        .all(db)
        .await?;

    let count = |status: Status| counts.iter().find(|(s, _)| *s == status).map_or(0, |(_, n)| *n as u64);

    let recent = delivery::Entity::find()
        .filter(delivery::Column::NewsletterId.eq(newsletter_id))
        .filter(delivery::Column::Status.eq(Status::Sent))
        .order_by_desc(delivery::Column::SentAt)
        .limit(RECENT)
        .all(db)
        .await?;

    let problems = delivery::Entity::find()
        .filter(delivery::Column::NewsletterId.eq(newsletter_id))
        .filter(delivery::Column::Status.is_in([Status::Failed, Status::Unknown]))
        .order_by_asc(delivery::Column::Email)
        .all(db)
        .await?;

    Ok(Progress {
        queued: count(Status::Queued),
        sending: count(Status::Sending),
        sent: count(Status::Sent),
        failed: count(Status::Failed),
        unknown: count(Status::Unknown),
        skipped: count(Status::Skipped),
        recent,
        problems,
    })
}

pub async fn run_worker(state: AppState) {
    let rate = send_rate(&state).await;

    loop {
        match work(&state, rate).await {
            Ok(true) => {}
            Ok(false) => tokio::time::sleep(IDLE).await,
            Err(e) => {
                eprintln!("[send worker] database error: {e}");
                tokio::time::sleep(IDLE * 5).await;
            }
        }
    }
}

async fn send_rate(state: &AppState) -> f64 {
    match state.ses.get_account().send().await {
        Ok(account) => account.send_quota().map(|quota| quota.max_send_rate()).filter(|rate| *rate > 0.0).unwrap_or(1.0),
        Err(e) => {
            eprintln!("[send worker] could not read the SES send rate, using 1 per second: {e}");
            1.0
        }
    }
}

async fn work(state: &AppState, rate: f64) -> Result<bool, DbErr> {
    sweep_stale(&state.db).await?;

    let next: Option<String> = delivery::Entity::find()
        .select_only()
        .column(delivery::Column::NewsletterId)
        .filter(delivery::Column::Status.eq(Status::Queued))
        .filter(due(now()))
        .into_tuple()
        .one(&state.db)
        .await?;

    let Some(newsletter_id) = next else { return Ok(false) };
    process(state, &newsletter_id, rate).await?;
    Ok(true)
}

async fn process(state: &AppState, newsletter_id: &str, rate: f64) -> Result<(), DbErr> {
    let Some((newsletter, Some(publication))) = Newsletter::find_by_id(newsletter_id)
        .find_also_related(Publication)
        .one(&state.db)
        .await?
    else {
        return Ok(());
    };

    let mailing = match NewsletterMailing::prepare(&state.ses, &newsletter, &publication, &state.urls).await {
        Ok(mailing) => mailing,
        Err(error) => return defer_newsletter(&state.db, newsletter_id, &error).await,
    };

    let result = send_batches(state, newsletter_id, &mailing, rate).await;

    if let Err(e) = mailing.finish().await {
        eprintln!("[send worker] newsletter {newsletter_id}: {e}");
    }

    result
}

async fn send_batches(state: &AppState, newsletter_id: &str, mailing: &NewsletterMailing<'_>, rate: f64) -> Result<(), DbErr> {
    loop {
        let claimed = claim(&state.db, newsletter_id).await?;
        if claimed.is_empty() {
            return Ok(());
        }

        let started = tokio::time::Instant::now();
        let by_token: HashMap<&str, &delivery::Model> = claimed.iter().map(|row| (row.subscriber_token.as_str(), row)).collect();

        let subscribers = Subscriber::find()
            .filter(subscriber::Column::Token.is_in(by_token.keys().copied()))
            .filter(subscriber::Column::IsConfirmed.eq(true))
            .all(&state.db)
            .await?;

        let present: HashSet<&str> = subscribers.iter().map(|subscriber| subscriber.token.as_str()).collect();
        let gone: Vec<&delivery::Model> = claimed.iter().filter(|row| !present.contains(row.subscriber_token.as_str())).collect();
        set_status(&state.db, &gone, Status::Skipped, Some("The subscriber left before the send")).await?;

        match mailing.send(&subscribers).await {
            Ok(deliveries) => {
                for delivery in deliveries {
                    let row = by_token[delivery.subscriber.token.as_str()];
                    match delivery.outcome {
                        Outcome::Sent(message_id) => mark_sent(&state.db, row, message_id).await?,
                        Outcome::Retry(message) => retry_later(&state.db, &[row], &message).await?,
                        Outcome::Failed(message) => set_status(&state.db, &[row], Status::Failed, Some(&message)).await?,
                    }
                }
            }
            Err(error) => {
                let rows: Vec<&delivery::Model> = subscribers.iter().map(|subscriber| by_token[subscriber.token.as_str()]).collect();
                if error.retryable {
                    retry_later(&state.db, &rows, &error.message).await?;
                    return Ok(());
                }
                set_status(&state.db, &rows, Status::Failed, Some(&error.message)).await?;
            }
        }

        tokio::time::sleep_until(started + Duration::from_secs_f64(subscribers.len() as f64 / rate)).await;
    }
}

async fn claim(db: &DatabaseConnection, newsletter_id: &str) -> Result<Vec<delivery::Model>, DbErr> {
    let at = now();

    let batch = Query::select()
        .columns([delivery::Column::NewsletterId, delivery::Column::Email])
        .from(delivery::Entity)
        .cond_where(
            Condition::all()
                .add(delivery::Column::NewsletterId.eq(newsletter_id))
                .add(delivery::Column::Status.eq(Status::Queued))
                .add(due(at)),
        )
        .order_by(delivery::Column::CreatedAt, Order::Asc)
        .order_by(delivery::Column::Email, Order::Asc)
        .limit(BATCH_SIZE as u64)
        .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
        .to_owned();

    delivery::Entity::update_many()
        .col_expr(delivery::Column::Status, Expr::value(Status::Sending))
        .col_expr(delivery::Column::ClaimedAt, Expr::value(at))
        .col_expr(delivery::Column::Attempts, Expr::col(delivery::Column::Attempts).add(1))
        .filter(
            Expr::tuple([Expr::col(delivery::Column::NewsletterId).into(), Expr::col(delivery::Column::Email).into()])
                .in_subquery(batch),
        )
        .exec_with_returning(db)
        .await
}

async fn defer_newsletter(db: &DatabaseConnection, newsletter_id: &str, error: &MailError) -> Result<(), DbErr> {
    let update = delivery::Entity::update_many()
        .col_expr(delivery::Column::Error, Expr::value(error.message.clone()))
        .filter(delivery::Column::NewsletterId.eq(newsletter_id))
        .filter(delivery::Column::Status.eq(Status::Queued));

    let update = match error.retryable {
        true => update.col_expr(delivery::Column::NextAttemptAt, Expr::value(now() + PREPARE_RETRY)),
        false => update.col_expr(delivery::Column::Status, Expr::value(Status::Failed)),
    };

    update.exec(db).await?;
    Ok(())
}

async fn retry_later(db: &impl ConnectionTrait, rows: &[&delivery::Model], message: &str) -> Result<(), DbErr> {
    let (exhausted, waiting): (Vec<&delivery::Model>, Vec<&delivery::Model>) =
        rows.iter().partition(|row| row.attempts >= MAX_ATTEMPTS);

    set_status(db, &exhausted, Status::Failed, Some(message)).await?;

    let mut by_attempts: BTreeMap<i32, Vec<&delivery::Model>> = BTreeMap::new();
    for row in waiting {
        by_attempts.entry(row.attempts).or_default().push(row);
    }

    for (attempts, rows) in by_attempts {
        rows_filter(delivery::Entity::update_many(), &rows)
            .col_expr(delivery::Column::Status, Expr::value(Status::Queued))
            .col_expr(delivery::Column::NextAttemptAt, Expr::value(now() + backoff(attempts)))
            .col_expr(delivery::Column::ClaimedAt, Expr::value(None::<Timestamp>))
            .col_expr(delivery::Column::Error, Expr::value(message.to_string()))
            .exec(db)
            .await?;
    }

    Ok(())
}

async fn set_status(db: &impl ConnectionTrait, rows: &[&delivery::Model], status: Status, error: Option<&str>) -> Result<(), DbErr> {
    if rows.is_empty() {
        return Ok(());
    }

    rows_filter(delivery::Entity::update_many(), rows)
        .col_expr(delivery::Column::Status, Expr::value(status))
        .col_expr(delivery::Column::Error, Expr::value(error.map(str::to_string)))
        .col_expr(delivery::Column::ClaimedAt, Expr::value(None::<Timestamp>))
        .exec(db)
        .await?;
    Ok(())
}

async fn mark_sent(db: &impl ConnectionTrait, row: &delivery::Model, message_id: Option<String>) -> Result<(), DbErr> {
    rows_filter(delivery::Entity::update_many(), &[row])
        .col_expr(delivery::Column::Status, Expr::value(Status::Sent))
        .col_expr(delivery::Column::SentAt, Expr::value(now()))
        .col_expr(delivery::Column::MessageId, Expr::value(message_id))
        .col_expr(delivery::Column::Error, Expr::value(None::<String>))
        .col_expr(delivery::Column::ClaimedAt, Expr::value(None::<Timestamp>))
        .exec(db)
        .await?;
    Ok(())
}

fn rows_filter(update: sea_orm::UpdateMany<delivery::Entity>, rows: &[&delivery::Model]) -> sea_orm::UpdateMany<delivery::Entity> {
    let newsletter_ids: HashSet<&str> = rows.iter().map(|row| row.newsletter_id.as_str()).collect();
    update
        .filter(delivery::Column::NewsletterId.is_in(newsletter_ids))
        .filter(delivery::Column::Email.is_in(rows.iter().map(|row| row.email.as_str())))
}

async fn sweep_stale(db: &DatabaseConnection) -> Result<(), DbErr> {
    delivery::Entity::update_many()
        .col_expr(delivery::Column::Status, Expr::value(Status::Unknown))
        .col_expr(delivery::Column::ClaimedAt, Expr::value(None::<Timestamp>))
        .col_expr(
            delivery::Column::Error,
            Expr::value("The server stopped during the send. This email may or may not have been sent."),
        )
        .filter(delivery::Column::Status.eq(Status::Sending))
        .filter(delivery::Column::ClaimedAt.lt(now() - STALE_CLAIM))
        .exec(db)
        .await?;
    Ok(())
}
