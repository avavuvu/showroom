use axum::{Json, extract::{Path, State}};
use boutique::{AppError, AppResult};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::Serialize;

use crate::{
    document::Document,
    models::newsletter::{self, Entity as Newsletter},
    renderer::markdown,
    services::subdomain::CurrentPublication,
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

pub async fn get_newsletters(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
) -> AppResult<Json<Vec<NewsletterSummary>>> {
    let newsletters = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::SentAt.is_not_null())
        .order_by_desc(newsletter::Column::SentAt)
        .all(&state.db)
        .await?;

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
    CurrentPublication(publication): CurrentPublication,
    Path(slug): Path<String>,
) -> AppResult<Json<NewsletterResponse>> {
    let newsletter = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .filter(newsletter::Column::SentAt.is_not_null())
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

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
        content: markdown::render(&Document::from_stored(&newsletter.content)),
    }))
}
