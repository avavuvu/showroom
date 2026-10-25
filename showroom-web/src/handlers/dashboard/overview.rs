use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use boutique::AppResult;
use nanoid::nanoid;
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, ModelTrait, QueryFilter, QueryOrder};
use crate::{
    document::Document,
    services::publication::{OwnedPublication, find_newsletter},
    models::newsletter::{self, Entity as Newsletter},
    state::AppState,
    views,
};

pub async fn index(
    State(state): State<AppState>,
    owned: OwnedPublication,
) -> AppResult<Markup> {
    let newsletters = Newsletter::find_summaries()
        .filter(newsletter::Column::PublicationId.eq(&owned.publication.id))
        .order_by_desc(newsletter::Column::UpdatedAt)
        .all(&state.db)
        .await?;

    Ok(views::dashboard::index(&owned.into_context(state.urls.clone()), newsletters))
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
        content: Set(Document::greeting().to_value()),
        revision: Set(0),
        published_at: Set(None),
        send_started_at: Set(None),
        email_subject: Set(None),
        email_html: Set(None),
        email_text: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        hero_image_id: Set(None)
    };

    let newsletter = new_newsletter.insert(&state.db).await?;

    Ok(Redirect::to(&format!("{}/edit/{}", state.urls.dashboard(&publication.slug), newsletter.id)))
}

pub async fn delete_newsletter(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<StatusCode> {
    let newsletter = find_newsletter(&state.db, &publication.id, &id).await?;

    newsletter.delete(&state.db).await?;

    Ok(StatusCode::OK)
}
