use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
};
use boutique::{AppError, AppResult};
use maud::Markup;
use nanoid::nanoid;
use std::collections::HashSet;

use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, TransactionTrait,
    sea_query::OnConflict,
};
use validator::ValidateEmail;

use crate::{
    services::publication::OwnedPublication,
    models::subscriber::{self, Entity as Subscriber},
    state::AppState,
    views::{self, dashboard::subscribers::ImportSummary},
};

const INSERT_CHUNK: usize = 1000;

async fn fetch_subscribers(publication_id: &str, db: &sea_orm::DatabaseConnection) -> Result<Vec<subscriber::Model>, sea_orm::DbErr> {
    Subscriber::find()
        .filter(subscriber::Column::PublicationId.eq(publication_id))
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .order_by_desc(subscriber::Column::CreatedAt)
        .all(db)
        .await
}

pub async fn get_subscribers(
    State(state): State<AppState>,
    owned: OwnedPublication,
) -> AppResult<Markup> {
    let subscribers = fetch_subscribers(&owned.publication.id, &state.db).await?;
    Ok(views::dashboard::subscribers::index(&owned.into_context(state.urls.clone()), &subscribers))
}

pub async fn import_subscribers(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    mut multipart: Multipart,
) -> AppResult {
    while let Some(field) = multipart.next_field().await.map_err(|_| AppError::BadRequest)? {
        if field.name() != Some("file") {
            continue;
        }

        let bytes = field.bytes().await.map_err(|_| AppError::BadRequest)?;
        let content = std::str::from_utf8(&bytes).map_err(|_| AppError::message("File must be UTF-8"))?;
        let mut reader = csv::Reader::from_reader(content.as_bytes());

        let headers = reader
            .headers()
            .map_err(|_| AppError::message("Could not read CSV headers"))?
            .clone();

        let column = |names: &[&str]| headers.iter().position(|header| names.iter().any(|name| header.trim().eq_ignore_ascii_case(name)));
        let email_index = column(&["email"]).ok_or_else(|| AppError::message("CSV must have an 'email' column"))?;
        let name_index = column(&["name"]);
        let date_index = column(&["created_at", "subscribed_at"]);

        let mut models = Vec::new();
        let mut seen = HashSet::new();
        let mut summary = ImportSummary::default();

        for record in reader.records() {
            let Ok(record) = record else {
                summary.invalid += 1;
                continue;
            };

            let email = record.get(email_index).map(|email| email.trim().to_lowercase()).unwrap_or_default();
            if !email.validate_email() {
                summary.invalid += 1;
                continue;
            }
            if !seen.insert(email.clone()) {
                summary.existing += 1;
                continue;
            }

            let name = name_index
                .and_then(|index| record.get(index))
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_string);

            let created_at = date_index
                .and_then(|index| record.get(index))
                .and_then(|date| chrono::DateTime::parse_from_rfc3339(date.trim()).ok())
                .unwrap_or_else(|| chrono::Utc::now().fixed_offset());

            models.push(subscriber::ActiveModel {
                token: Set(nanoid!(21)),
                publication_id: Set(publication.id.clone()),
                name: Set(name),
                email: Set(email),
                is_confirmed: Set(true),
                created_at: Set(created_at),
            });
        }

        let transaction = state.db.begin().await?;
        let mut models = models.into_iter().peekable();
        while models.peek().is_some() {
            let chunk: Vec<_> = models.by_ref().take(INSERT_CHUNK).collect();
            let attempted = chunk.len();
            let inserted = Subscriber::insert_many(chunk)
                .on_conflict(
                    OnConflict::columns([subscriber::Column::PublicationId, subscriber::Column::Email])
                        .do_nothing()
                        .to_owned(),
                )
                .exec_without_returning(&transaction)
                .await? as usize;
            summary.imported += inserted;
            summary.existing += attempted - inserted;
        }
        transaction.commit().await?;

        let subscribers = fetch_subscribers(&publication.id, &state.db).await?;
        return Ok(views::dashboard::subscribers::import_result(&subscribers, summary).into_response());
    }

    Err(AppError::message("No file received"))
}
