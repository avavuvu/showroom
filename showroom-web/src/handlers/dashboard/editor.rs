use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use boutique::{AppError, AppResult, AuthenticatedUser};
use chrono::Utc;
use maud::Markup;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::Expr,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    document::Document,
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::Entity as Publication,
        user,
    },
    services::publication::OwnedPublication,
    state::AppState,
    views,
};

pub async fn get_edit(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Markup> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&owned.publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(views::dashboard::edit(&owned.into_context(state.urls.clone()), &newsletter))
}

async fn find_owned_newsletter(id: &str, user_id: &str, db: &DatabaseConnection) -> AppResult<newsletter::Model> {
    let (newsletter, publication) = Newsletter::find_by_id(id)
        .find_also_related(Publication)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    match publication {
        Some(p) if p.owner_id == user_id => Ok(newsletter),
        _ => Err(AppError::NotFound),
    }
}

#[derive(Deserialize)]
pub struct SaveRequest {
    title: String,
    subtitle: Option<String>,
    content: Value,
    revision: i32,
}

#[derive(Serialize)]
pub struct SaveResponse {
    revision: i32,
}

pub async fn put_edit_json(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    Path(id): Path<String>,
    Json(body): Json<SaveRequest>,
) -> AppResult<Response> {
    let newsletter = find_owned_newsletter(&id, &user.id, &state.db).await?;

    let document = Document::from_value(&body.content)
        .map_err(|error| AppError::message(format!("The newsletter content is not valid: {error}")))?;

    let subtitle = body.subtitle.map(|subtitle| subtitle.trim().to_string()).filter(|subtitle| !subtitle.is_empty());

    let result = Newsletter::update_many()
        .col_expr(newsletter::Column::Title, Expr::value(body.title.trim().to_string()))
        .col_expr(newsletter::Column::Subtitle, Expr::value(subtitle))
        .col_expr(newsletter::Column::Content, Expr::value(document.to_value()))
        .col_expr(newsletter::Column::UpdatedAt, Expr::value(Utc::now().fixed_offset()))
        .col_expr(newsletter::Column::Revision, Expr::col(newsletter::Column::Revision).add(1))
        .filter(newsletter::Column::Id.eq(&newsletter.id))
        .filter(newsletter::Column::Revision.eq(body.revision))
        .exec(&state.db)
        .await?;

    if result.rows_affected == 0 {
        let current = Newsletter::find_by_id(&newsletter.id)
            .one(&state.db)
            .await?
            .ok_or(AppError::NotFound)?;
        return Ok((StatusCode::CONFLICT, Json(SaveResponse { revision: current.revision })).into_response());
    }

    Ok(Json(SaveResponse { revision: body.revision + 1 }).into_response())
}
