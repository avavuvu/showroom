use axum::{Json, extract::{Path, State}, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    models::{
        newsletter::{self, Entity as Newsletter},
        user::{self, Entity as User},
    },
    renderer::markdown,
    services::subdomain::UsernameSubdomain,
    state::AppState,
};

#[derive(Serialize)]
pub struct NewsletterResponse {
    pub title: String,
    pub subtitle: Option<String>,
    pub date: String,
    pub content: String,
}

pub async fn get_newsletter(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Path(slug): Path<String>,
) -> Result<Json<NewsletterResponse>, StatusCode> {
    let owner = User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let newsletter = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&owner.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .filter(newsletter::Column::SentAt.is_not_null())
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let date = newsletter
        .sent_at
        .expect("filtered by SentAt.is_not_null()")
        .format("%Y-%m-%d")
        .to_string();

    Ok(Json(NewsletterResponse {
        title: newsletter.title,
        subtitle: newsletter.subtitle,
        date,
        content: markdown::render(&newsletter.content),
    }))
}
