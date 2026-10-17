use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::Duration,
};

use tokio::time::Instant;

use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, Order, QueryFilter, QueryOrder,
    QuerySelect, Statement, TransactionTrait,
    sea_query::{Expr, LockBehavior, LockType, Query},
};

use crate::{
    mailer::{BATCH_SIZE, MailError, NewsletterMailing, Outcome, send_newsletter::{self, RenderedEmail}},
    models::{
        delivery::{self, Status},
        newsletter::{self, Entity as Newsletter},
        publication::Entity as Publication,
        subscriber::{self, Entity as Subscriber},
    },
    state::AppState,
};

const MAX_ATTEMPTS: i32 = 4;
const SWEEP_EVERY: Duration = Duration::from_secs(5 * 60);
const ERROR_PAUSE: Duration = Duration::from_secs(10);
const PREPARE_RETRY: chrono::Duration = chrono::Duration::seconds(60);
const STALE_CLAIM: chrono::Duration = chrono::Duration::minutes(10);
const RECENT: u64 = 8;
const PROBLEMS_SHOWN: u64 = 100;
const RESTART_PAUSE: Duration = Duration::from_secs(5);

type Timestamp = DateTime<FixedOffset>;

fn now() -> Timestamp {
    Utc::now().fixed_offset()
}

fn due(at: Timestamp) -> Condition {
    Condition::any()
        .add(delivery::Column::NextAttemptAt.is_null())
        .add(delivery::Column::NextAttemptAt.lte(at))
}

fn backoff(attempts: i32) -> chrono::Duration {
    chrono::Duration::seconds(30 << (attempts - 1).clamp(0, 10))
}

pub async fn enqueue(
    db: &DatabaseConnection,
    newsletter: &newsletter::Model,
    email: RenderedEmail,
    started_at: Timestamp,
) -> Result<bool, DbErr> {
    let transaction = db.begin().await?;

    let started = Newsletter::update_many()
        .col_expr(newsletter::Column::PublishedAt, Expr::value(started_at))
        .col_expr(newsletter::Column::SendStartedAt, Expr::value(started_at))
        .col_expr(newsletter::Column::EmailSubject, Expr::value(email.subject))
        .col_expr(newsletter::Column::EmailHtml, Expr::value(email.html))
        .col_expr(newsletter::Column::EmailText, Expr::value(email.text))
        .filter(newsletter::Column::Id.eq(&newsletter.id))
        .filter(newsletter::Column::PublishedAt.is_null())
        .filter(newsletter::Column::SendStartedAt.is_null())
        .exec(&transaction)
        .await?
        .rows_affected
        == 1;

    if !started {
        transaction.rollback().await?;
        return Ok(false);
    }

    transaction
        .execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO newsletter_deliveries (newsletter_id, email, subscriber_token, status, attempts, created_at) \
             SELECT DISTINCT ON (lower(trim(email))) $1, lower(trim(email)), token, $2, 0, $3 \
             FROM subscribers WHERE publication_id = $4 AND is_confirmed \
             ORDER BY lower(trim(email)), created_at \
             ON CONFLICT (newsletter_id, email) DO NOTHING",
            [newsletter.id.clone().into(), "queued".into(), started_at.into(), newsletter.publication_id.clone().into()],
        ))
        .await?;

    transaction.commit().await?;
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
        .limit(PROBLEMS_SHOWN)
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

pub async fn supervise_worker(state: AppState) {
    loop {
        match tokio::spawn(run_worker(state.clone())).await {
            Err(error) if error.is_panic() => {
                tracing::error!("send worker: crashed, restarting: {error}");
                tokio::time::sleep(RESTART_PAUSE).await;
            }
            _ => return,
        }
    }
}

async fn run_worker(state: AppState) {
    let rate = send_rate(&state).await;
    let mut swept_at: Option<Instant> = None;

    loop {
        if swept_at.is_none_or(|at| at.elapsed() >= SWEEP_EVERY) {
            match sweep_stale(&state.db).await {
                Ok(()) => swept_at = Some(Instant::now()),
                Err(e) => tracing::error!("send worker: sweeping stale claims failed: {e}"),
            }
        }

        match work(&state, rate).await {
            Ok(true) => {}
            Ok(false) => idle(&state, swept_at).await,
            Err(e) => {
                tracing::error!("send worker: database error: {e}");
                tokio::time::sleep(ERROR_PAUSE).await;
            }
        }
    }
}

async fn idle(state: &AppState, swept_at: Option<Instant>) {
    let until_sweep = swept_at.map_or(ERROR_PAUSE, |at| SWEEP_EVERY.saturating_sub(at.elapsed()));

    let until_due = match next_attempt(&state.db).await {
        Ok(next) => next.map(|at| (at - now()).to_std().unwrap_or(Duration::ZERO)),
        Err(e) => {
            tracing::error!("send worker: reading the next retry time failed: {e}");
            Some(ERROR_PAUSE)
        }
    };

    let wait = until_due.map_or(until_sweep, |due| due.min(until_sweep));

    tokio::select! {
        _ = state.sends.notified() => {}
        _ = tokio::time::sleep(wait) => {}
    }
}

async fn next_attempt(db: &DatabaseConnection) -> Result<Option<Timestamp>, DbErr> {
    let next: Option<Option<Timestamp>> = delivery::Entity::find()
        .select_only()
        .column_as(Expr::col(delivery::Column::NextAttemptAt).min(), "next")
        .filter(delivery::Column::Status.eq(Status::Queued))
        .into_tuple()
        .one(db)
        .await?;
    Ok(next.flatten())
}

async fn send_rate(state: &AppState) -> f64 {
    match state.ses.get_account().send().await {
        Ok(account) => account.send_quota().map(|quota| quota.max_send_rate()).filter(|rate| *rate > 0.0).unwrap_or(1.0),
        Err(e) => {
            tracing::warn!("send worker: could not read the SES send rate, using 1 per second: {e}");
            1.0
        }
    }
}

async fn work(state: &AppState, rate: f64) -> Result<bool, DbErr> {
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

    let email = match snapshot(&newsletter) {
        Some(email) => email,
        None => {
            let date = newsletter.send_started_at.unwrap_or(newsletter.created_at);
            match send_newsletter::render(&newsletter, &publication, date, &state.urls) {
                Ok(email) => email,
                Err(error) => return defer_newsletter(&state.db, newsletter_id, &error).await,
            }
        }
    };

    let mailing = match NewsletterMailing::prepare(&state.ses, newsletter_id, &email, &publication, &state.urls).await {
        Ok(mailing) => mailing,
        Err(error) => return defer_newsletter(&state.db, newsletter_id, &error).await,
    };

    let result = send_batches(state, newsletter_id, &mailing, rate).await;

    if let Err(e) = mailing.finish().await {
        tracing::error!("send worker: newsletter {newsletter_id}: {e}");
    }

    result
}

fn snapshot(newsletter: &newsletter::Model) -> Option<RenderedEmail> {
    Some(RenderedEmail {
        subject: newsletter.email_subject.clone()?,
        html: newsletter.email_html.clone()?,
        text: newsletter.email_text.clone()?,
    })
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
                let transaction = state.db.begin().await?;
                for delivery in deliveries {
                    let row = by_token[delivery.subscriber.token.as_str()];
                    match delivery.outcome {
                        Outcome::Sent(message_id) => mark_sent(&transaction, row, message_id).await?,
                        Outcome::Retry(message) => retry_later(&transaction, &[row], &message).await?,
                        Outcome::Failed(message) => set_status(&transaction, &[row], Status::Failed, Some(&message)).await?,
                    }
                }
                transaction.commit().await?;
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
    let queued = || {
        delivery::Entity::update_many()
            .col_expr(delivery::Column::Error, Expr::value(error.message.clone()))
            .filter(delivery::Column::NewsletterId.eq(newsletter_id))
            .filter(delivery::Column::Status.eq(Status::Queued))
    };

    let transaction = db.begin().await?;
    if error.retryable {
        queued()
            .col_expr(delivery::Column::Status, Expr::value(Status::Failed))
            .filter(delivery::Column::Attempts.gte(MAX_ATTEMPTS))
            .exec(&transaction)
            .await?;
        queued()
            .col_expr(delivery::Column::Attempts, Expr::col(delivery::Column::Attempts).add(1))
            .col_expr(delivery::Column::NextAttemptAt, Expr::value(now() + PREPARE_RETRY))
            .exec(&transaction)
            .await?;
    } else {
        queued().col_expr(delivery::Column::Status, Expr::value(Status::Failed)).exec(&transaction).await?;
    }
    transaction.commit().await
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
