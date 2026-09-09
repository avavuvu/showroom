use axum::{Json, extract::{Path, State}, http::StatusCode};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use serde_json::Value;
use crate::{
    auth::extractors::{AuthenticatedUser, OwnedPublication},
    models::{newsletter::{self, Entity as Newsletter}, publication::Entity as Publication},
    state::AppState,
    views,
};

pub async fn get_edit(
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

    Ok(views::dashboard::edit(&owned.into_context(state.urls.clone()), &newsletter))
}

async fn find_owned_newsletter(id: &str, user_id: &str, db: &DatabaseConnection) -> Result<newsletter::Model, StatusCode> {
    let (newsletter, publication) = Newsletter::find_by_id(id)
        .find_also_related(Publication)
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    match publication {
        Some(p) if p.owner_id == user_id => Ok(newsletter),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_edit_json(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<NewsletterResponse>, StatusCode> {
    let newsletter = find_owned_newsletter(&id, &user.id, &state.db).await?;

    let response = NewsletterResponse {
        title: newsletter.title,
        subtitle: newsletter.subtitle,
        content: newsletter.content,
    };

    Ok(Json(response))
}

#[derive(Serialize, Deserialize)]
pub struct NewsletterResponse {
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
    let newsletter = find_owned_newsletter(&id, &user.id, &state.db).await?;

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
