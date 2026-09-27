use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use boutique::{AppError, AppResult};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use crate::{
    services::publication::OwnedPublication,
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
) -> AppResult<Markup> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&owned.publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(views::dashboard::preview(&owned.into_context(state.urls.clone()), &newsletter))
}

pub async fn post_send(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    if newsletter.sent_at.is_some() {
        return Err(AppError::Status(StatusCode::CONFLICT));
    }

    let subscribers = publication
        .find_related(Subscriber)
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .all(&state.db)
        .await?;

    mailer::send_newsletter(&state.ses, &newsletter, &publication, &subscribers, &state.urls)
        .await
        .map_err(|e| AppError::internal("send newsletter", e))?;

    let url = format!("{}/{}", state.urls.publication(&publication.slug), newsletter.slug);

    let mut active: newsletter::ActiveModel = newsletter.into();
    active.sent_at = Set(Some(chrono::Utc::now().fixed_offset()));
    active.update(&state.db).await?;

    Ok(Redirect::to(&url))
}
