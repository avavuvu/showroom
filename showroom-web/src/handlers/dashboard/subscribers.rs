use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
};
use boutique::{AppError, AppResult};
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
    sea_query::OnConflict,
};

use crate::{
    services::publication::OwnedPublication,
    models::subscriber::{self, Entity as Subscriber},
    state::AppState,
    views,
};

async fn fetch_subscribers(publication_id: &str, db: &sea_orm::DatabaseConnection) -> Vec<subscriber::Model> {
    Subscriber::find()
        .filter(subscriber::Column::PublicationId.eq(publication_id))
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .order_by_desc(subscriber::Column::CreatedAt)
        .all(db)
        .await
        .unwrap_or_default()
}

pub async fn get_subscribers(
    State(state): State<AppState>,
    owned: OwnedPublication,
) -> Markup {
    let subscribers = fetch_subscribers(&owned.publication.id, &state.db).await;
    views::dashboard::subscribers::index(&owned.into_context(state.urls.clone()), &subscribers)
}

pub async fn import_subscribers(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    mut multipart: Multipart,
) -> AppResult {
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("file") {
            continue;
        }

        let bytes = field.bytes().await.map_err(|_| AppError::BadRequest)?;

        let content = String::from_utf8(bytes.to_vec())
            .map_err(|_| AppError::message("File must be UTF-8"))?;

        let mut reader = csv::Reader::from_reader(content.as_bytes());

        let headers = reader
            .headers()
            .map_err(|_| AppError::message("Could not read CSV headers"))?
            .clone();

        let email_index = headers.iter().position(|h| h.trim().eq_ignore_ascii_case("email"));
        let name_index  = headers.iter().position(|h| h.trim().eq_ignore_ascii_case("name"));
        let date_index  = headers.iter().position(|h| {
            let h = h.trim();
            h.eq_ignore_ascii_case("created_at") || h.eq_ignore_ascii_case("subscribed_at")
        });

        let email_index = email_index.ok_or_else(|| AppError::message("CSV must have an 'email' column"))?;

        let mut models  = Vec::new();
        let mut skipped = 0usize;

        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(_) => { skipped += 1; continue; }
            };

            let email = match record.get(email_index).map(str::trim).filter(|s| !s.is_empty()) {
                Some(e) => e.to_string(),
                None => { skipped += 1; continue; }
            };

            let name = name_index
                .and_then(|i| record.get(i))
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);

            let created_at = date_index
                .and_then(|i| record.get(i))
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s.trim()).ok())
                .unwrap_or_else(|| chrono::Utc::now().fixed_offset());

            models.push(subscriber::ActiveModel {
                token:          Set(nanoid!(21)),
                publication_id: Set(publication.id.clone()),
                name:         Set(name),
                email:        Set(email),
                is_confirmed: Set(true),
                created_at:   Set(created_at),
            });
        }

        if !models.is_empty() {
            let _ = Subscriber::insert_many(models)
                .on_conflict(
                    OnConflict::columns([subscriber::Column::PublicationId, subscriber::Column::Email])
                        .do_nothing()
                        .to_owned()
                )
                .exec(&state.db)
                .await;
        }

        let subscribers = fetch_subscribers(&publication.id, &state.db).await;
        return Ok(views::dashboard::subscribers::import_result(&subscribers, skipped).into_response());
    }

    Err(AppError::message("No file received"))
}
