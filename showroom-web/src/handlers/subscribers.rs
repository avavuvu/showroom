use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
    sea_query::OnConflict,
};

use crate::{
    auth::extractors::AuthenticatedUser,
    models::subscriber::{self, Entity as Subscriber},
    state::AppState,
    views::{self, PageContext},
};

async fn fetch_subscribers(user_id: &str, db: &sea_orm::DatabaseConnection) -> Vec<subscriber::Model> {
    Subscriber::find()
        .filter(subscriber::Column::UserId.eq(user_id))
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .order_by_desc(subscriber::Column::CreatedAt)
        .all(db)
        .await
        .unwrap_or_default()
}

pub async fn get_subscribers(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let subscribers = fetch_subscribers(&user.id, &state.db).await;
    views::subscribers::index(&PageContext::from_user(&user, state.urls.clone()), &subscribers)
}



pub async fn import_subscribers(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    mut multipart: Multipart,
) -> Response {
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("file") {
            continue;
        }

        let bytes = match field.bytes().await {
            Ok(b) => b,
            Err(_) => return StatusCode::BAD_REQUEST.into_response(),
        };

        let content = match String::from_utf8(bytes.to_vec()) {
            Ok(s) => s,
            Err(_) => return (StatusCode::BAD_REQUEST, "File must be UTF-8").into_response(),
        };

        let mut reader = csv::Reader::from_reader(content.as_bytes());

        let headers = match reader.headers() {
            Ok(h) => h.clone(),
            Err(_) => return (StatusCode::BAD_REQUEST, "Could not read CSV headers").into_response(),
        };

        let email_index = headers.iter().position(|h| h.trim().eq_ignore_ascii_case("email"));
        let name_index  = headers.iter().position(|h| h.trim().eq_ignore_ascii_case("name"));
        let date_index  = headers.iter().position(|h| {
            let h = h.trim();
            h.eq_ignore_ascii_case("created_at") || h.eq_ignore_ascii_case("subscribed_at")
        });

        let email_index = match email_index {
            Some(i) => i,
            None => return (StatusCode::BAD_REQUEST, "CSV must have an 'email' column").into_response(),
        };

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
                token:        Set(nanoid!(21)),
                user_id:      Set(user.id.clone()),
                name:         Set(name),
                email:        Set(email),
                is_confirmed: Set(true),
                created_at:   Set(created_at),
            });
        }

        if !models.is_empty() {
            let _ = Subscriber::insert_many(models)
                .on_conflict(
                    OnConflict::columns([subscriber::Column::UserId, subscriber::Column::Email])
                        .do_nothing()
                        .to_owned()
                )
                .exec(&state.db)
                .await;
        }

        let subscribers = fetch_subscribers(&user.id, &state.db).await;
        return views::subscribers::import_result(&subscribers, skipped).into_response();
    }

    (StatusCode::BAD_REQUEST, "No file received").into_response()
}
