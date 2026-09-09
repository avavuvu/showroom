use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use crate::{
    auth::extractors::OwnedPublication,
    models::newsletter::{self, Entity as Newsletter},
    models::subscriber::{self, Entity as Subscriber},
    mailer,
    state::AppState,
    views,
};

pub async fn get_send(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> Result<Markup, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&owned.publication.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(views::dashboard::preview(&owned.into_context(state.urls.clone()), &newsletter))
}

pub async fn post_send(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> Result<Redirect, StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if newsletter.sent_at.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let subscribers = publication
        .find_related(Subscriber)
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    mailer::send_newsletter(&state.ses, &newsletter, &publication, &subscribers, &state.urls)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = format!("{}/{}", state.urls.publication(&publication.slug), newsletter.slug);

    let mut active: newsletter::ActiveModel = newsletter.into();
    active.sent_at = Set(Some(chrono::Utc::now().fixed_offset()));
    active.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&url))
}
