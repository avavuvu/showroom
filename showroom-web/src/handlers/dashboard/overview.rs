use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use boutique::{AppError, AppResult};
use nanoid::nanoid;
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use crate::{
    document::Document,
    services::publication::OwnedPublication,
    models::newsletter::{self, Entity as Newsletter},
    state::AppState,
    views,
};

pub async fn index(
    State(state): State<AppState>,
    owned: OwnedPublication,
) -> Markup {
    let mut newsletters = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&owned.publication.id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    newsletters.sort_unstable_by_key(|n| std::cmp::Reverse(n.updated_at));

    views::dashboard::index(&owned.into_context(state.urls.clone()), newsletters)
}

pub async fn post_newsletters(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
) -> AppResult<Redirect> {
    let id = nanoid!(14);
    let now = chrono::Utc::now().fixed_offset();

    let new_newsletter = newsletter::ActiveModel {
        id: Set(id.clone()),
        publication_id: Set(publication.id.clone()),
        title: Set(String::new()),
        slug: Set(id.clone()),
        subtitle: Set(None),
        content: Set(Document::default().to_value()),
        revision: Set(0),
        sent_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let newsletter = new_newsletter.insert(&state.db).await?;

    Ok(Redirect::to(&format!("{}/edit/{}", state.urls.dashboard(&publication.slug), newsletter.id)))
}

pub async fn delete_newsletter(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<StatusCode> {
    let newsletter = Newsletter::find_by_id(&id)
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    newsletter.delete(&state.db).await?;

    Ok(StatusCode::OK)
}
