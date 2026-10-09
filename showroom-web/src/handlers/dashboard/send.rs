use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};
use boutique::{AppError, AppResult};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use slugify::slugify;
use crate::{
    services::{newsletter_send, publication::OwnedPublication},
    models::newsletter::{self, Entity as Newsletter},
    state::AppState,
    views,
};

pub async fn get_send(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Markup> {
    let newsletter = find(&state.db, &owned.publication.id, &id).await?;
    let ctx = owned.into_context(state.urls.clone());

    if newsletter.send_started_at.is_some() {
        let progress = newsletter_send::progress(&state.db, &newsletter.id).await?;
        return Ok(views::dashboard::sending::page(&ctx, &newsletter, &progress));
    }

    Ok(views::dashboard::preview(&ctx, &newsletter))
}

pub async fn get_progress(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Markup> {
    let newsletter = find(&state.db, &owned.publication.id, &id).await?;
    let progress = newsletter_send::progress(&state.db, &newsletter.id).await?;
    Ok(views::dashboard::sending::panel(&owned.into_context(state.urls.clone()), &newsletter, &progress))
}

pub async fn post_retry(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    let newsletter = find(&state.db, &owned.publication.id, &id).await?;
    newsletter_send::retry_failed(&state.db, &newsletter.id).await?;
    Ok(Redirect::to(&send_url(&state, &owned.publication.slug, &newsletter.id)))
}

pub async fn post_send(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<Redirect> {
    let newsletter = find(&state.db, &publication.id, &id).await?;

    if newsletter.send_started_at.is_some() {
        return Ok(Redirect::to(&send_url(&state, &publication.slug, &newsletter.id)));
    }

    if newsletter.published_at.is_some() {
        return Err(AppError::Status(StatusCode::CONFLICT));
    }

    if newsletter.title.trim().is_empty() {
        return Err(AppError::message("Add a title before you send this newsletter."));
    }

    let slug = unique_slug(&newsletter, &state.db).await?;
    let mut active: newsletter::ActiveModel = newsletter.into();
    active.slug = Set(slug);
    let newsletter = active.update(&state.db).await?;

    newsletter_send::enqueue(&state.db, &newsletter, &publication).await?;

    Ok(Redirect::to(&send_url(&state, &publication.slug, &newsletter.id)))
}

async fn find(db: &DatabaseConnection, publication_id: &str, id: &str) -> AppResult<newsletter::Model> {
    Newsletter::find_by_id(id)
        .filter(newsletter::Column::PublicationId.eq(publication_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)
}

fn send_url(state: &AppState, slug: &str, id: &str) -> String {
    format!("{}/send/{}", state.urls.dashboard(slug), id)
}

async fn unique_slug(newsletter: &newsletter::Model, db: &DatabaseConnection) -> AppResult<String> {
    let base = slugify(newsletter.title.trim(), "", "-", Some(80));
    if base.is_empty() {
        return Ok(newsletter.id.clone());
    }

    for attempt in 1..=20 {
        let candidate = if attempt == 1 { base.clone() } else { format!("{base}-{attempt}") };
        let taken = Newsletter::find()
            .filter(newsletter::Column::PublicationId.eq(&newsletter.publication_id))
            .filter(newsletter::Column::Slug.eq(&candidate))
            .filter(newsletter::Column::Id.ne(&newsletter.id))
            .count(db)
            .await?;
        if taken == 0 {
            return Ok(candidate);
        }
    }

    Ok(format!("{base}-{}", newsletter.id))
}
