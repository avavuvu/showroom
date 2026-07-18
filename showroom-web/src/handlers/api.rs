use axum::{Json, extract::{Path, State}, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
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
pub struct NewsletterSummary {
    pub title: String,
    pub subtitle: Option<String>,
    pub date: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct NewsletterResponse {
    pub title: String,
    pub subtitle: Option<String>,
    pub date: String,
    pub slug: String,
    pub content: String,
}

async fn find_owner(handle: &str, state: &AppState) -> Result<crate::models::user::Model, StatusCode> {
    User::find()
        .filter(user::Column::Handle.eq(handle))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_newsletters(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
) -> Result<Json<Vec<NewsletterSummary>>, StatusCode> {
    let owner = find_owner(&handle, &state).await?;

    let newsletters = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&owner.id))
        .filter(newsletter::Column::SentAt.is_not_null())
        .order_by_desc(newsletter::Column::SentAt)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let summaries = newsletters.into_iter().map(|n| NewsletterSummary {
        title: n.title,
        subtitle: n.subtitle,
        date: n.sent_at.expect("filtered by SentAt.is_not_null()").format("%Y-%m-%d").to_string(),
        slug: n.slug,
    }).collect();

    Ok(Json(summaries))
}

pub async fn get_newsletter(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Path(slug): Path<String>,
) -> Result<Json<NewsletterResponse>, StatusCode> {
    let owner = find_owner(&handle, &state).await?;

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
        slug: newsletter.slug,
        content: markdown::render(&newsletter.content),
    }))
}
