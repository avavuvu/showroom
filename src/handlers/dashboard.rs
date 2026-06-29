use axum::{Json, extract::{Path, State}, http::StatusCode, response::Redirect};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use serde_json::Value;
use crate::{
    auth::extractors::AuthenticatedUser,
    models::newsletter::{self, Entity as Newsletter},
    models::subscriber::{self, Entity as Subscriber},
    renderer::html::{ThemeVariables, render_email},
    mailer,
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

pub async fn get_send(
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

    let rendered = render_email(&newsletter.content, ThemeVariables::default());

    let mut active: newsletter::ActiveModel = newsletter.into();
    active.rendered = Set(Some(rendered));

    let rendered_newsletter = active.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let app_url = state.urls.app();

    Ok(views::dashboard::preview(&app_url, &rendered_newsletter))
}

pub async fn post_send(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::UserId.eq(&user.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if newsletter.sent_at.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let subscribers = user
        .find_related(Subscriber)
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    mailer::send_newsletter(&state.ses, &newsletter, &user.handle, &subscribers, &state.urls)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = format!("{}/{}", state.urls.user(&user.handle), newsletter.slug);

    let mut active: newsletter::ActiveModel = newsletter.into();
    active.sent_at = Set(Some(chrono::Utc::now().fixed_offset()));
    active.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&url))
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

    let app_url = state.urls.app();

    Ok(views::dashboard::edit(&app_url, &newsletter))
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
