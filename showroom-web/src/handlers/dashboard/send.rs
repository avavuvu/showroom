use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use crate::{
    auth::extractors::AuthenticatedUser,
    models::newsletter::{self, Entity as Newsletter},
    models::subscriber::{self, Entity as Subscriber},
    mailer,
    state::AppState,
    views::{self, PageContext},
};

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

    let page_ctx = PageContext::from_user(&user, state.urls.clone());

    Ok(views::dashboard::preview(&page_ctx, &newsletter))
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
