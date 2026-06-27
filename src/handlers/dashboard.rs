use axum::{Json, extract::{Path, State}, http::StatusCode};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use serde_json::Value;
use crate::{
    auth::extractors::AuthenticatedUser,
    models::newsletter::{self, Entity as Newsletter},
    renderer::html::ProseVars,
    state::AppState,
    views,
};

pub async fn index(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Markup {
    views::dashboard::index(&user, &state.urls.base())
}

pub async fn get_newsletters(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let mut newsletters = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&user.id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    newsletters.sort_unstable_by_key(|n| std::cmp::Reverse(n.updated_at));

    let user_base = state.urls.user(&user.handle);
    views::dashboard::newsletters(newsletters, &user_base)
}

pub async fn get_preview(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(slug): Path<String>,
) -> Result<Markup, StatusCode> {
    let newsletter = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&user.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(views::dashboard::preview(&newsletter, ProseVars::default()))
}

pub async fn get_edit(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Markup, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::UserId.eq(&user.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(views::dashboard::edit(&newsletter))
}

pub async fn get_edit_json(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<NewsletterResponse>, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::UserId.eq(&user.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let response = NewsletterResponse {
        title: newsletter.title,
        subtitle: newsletter.subtitle,
        content: newsletter.content,
    };

    Ok(Json(response))
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NewsletterResponse {
    title: String,
    subtitle: Option<String>,
    content: Value,
}

pub async fn put_edit_json(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(body): Json<NewsletterResponse>,
) -> Result<StatusCode, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::UserId.eq(&user.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active: newsletter::ActiveModel = newsletter.into();
    active.title = Set(body.title.clone());
    active.subtitle = Set(body.subtitle);
    active.slug = Set(slugify(&body.title, "", "-", None));
    active.content = Set(body.content);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());
    active.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
