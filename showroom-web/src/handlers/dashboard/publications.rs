use axum::{
    Form, extract::State, http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;

use crate::{
    auth::extractors::{AuthenticatedUser, OwnedPublication},
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        subscriber::{self, Entity as Subscriber},
        user,
    },
    state::AppState,
    views::{self, PageContext},
};

async fn account_context(user: &user::Model, state: &AppState) -> PageContext {
    let publications = publication::for_owner(&user.id, &state.db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("[publications] failed to load publications for {}: {e}", user.id);
            Vec::new()
        });
    PageContext::from_user(user, state.urls.clone()).with_publications(publications)
}

pub async fn index(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Response {
    let publications = match publication::for_owner(&user.id, &state.db).await {
        Ok(publications) => publications,
        Err(e) => {
            eprintln!("[publications] failed to load publications for {}: {e}", user.id);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match publications.iter().find(|p| p.is_default).or(publications.first()) {
        Some(room) => Redirect::to(&state.urls.dashboard(&room.slug)).into_response(),
        None => Redirect::to(&format!("{}/new", state.urls.app())).into_response(),
    }
}

#[derive(Deserialize)]
pub struct NewPublicationForm {
    pub slug: String,
    pub name: String,
}

pub async fn new_form(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    views::dashboard::publications::new_form(&account_context(&user, &state).await, "", "", None)
}

pub async fn create(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Form(form): Form<NewPublicationForm>,
) -> Response {
    let ctx = account_context(&user, &state).await;
    let slug = form.slug.trim().to_lowercase();
    let name = form.name.trim().to_string();

    let retry = |message: &str| views::dashboard::publications::new_form(&ctx, &slug, &name, Some(message)).into_response();

    if name.is_empty() {
        return retry("A name is required");
    }

    if let Err(message) = publication::validate_slug(&slug) {
        return retry(message);
    }

    let taken = Publication::find()
        .filter(publication::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .ok()
        .flatten()
        .is_some();

    if taken {
        return retry("This address is already taken");
    }

    let now = chrono::Utc::now().fixed_offset();
    let new_publication = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: Set(user.id.clone()),
        slug: Set(slug.clone()),
        name: Set(name.clone()),
        description: Set(None),
        theme: Set(None),
        is_default: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    match new_publication.insert(&state.db).await {
        Ok(_) => Redirect::to(&state.urls.dashboard(&slug)).into_response(),
        Err(e) => {
            eprintln!("[publications] create failed: {e}");
            retry("Something went wrong, please try again")
        }
    }
}


pub async fn get_settings(
    State(state): State<AppState>,
    owned: OwnedPublication,
) -> Markup {
    views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), None)
}

#[derive(Deserialize)]
pub struct SettingsForm {
    pub name: String,
    pub description: Option<String>,
}

pub async fn update_settings(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Form(form): Form<SettingsForm>,
) -> Response {
    let name = form.name.trim().to_string();
    let description = form.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());

    if name.is_empty() {
        return views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some("A name is required")).into_response();
    }

    let slug = owned.publication.slug.clone();
    let mut active: publication::ActiveModel = owned.publication.into();
    active.name = Set(name);
    active.description = Set(description);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    match active.update(&state.db).await {
        Ok(_) => Redirect::to(&format!("{}/settings", state.urls.dashboard(&slug))).into_response(),
        Err(e) => {
            eprintln!("[publications] update failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
) -> Response {
    if publication.is_default {
        return (StatusCode::FORBIDDEN, "Your room cannot be deleted").into_response();
    }

    let result = state.db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            Subscriber::delete_many()
                .filter(subscriber::Column::PublicationId.eq(&publication.id))
                .exec(txn)
                .await?;
            Newsletter::delete_many()
                .filter(newsletter::Column::PublicationId.eq(&publication.id))
                .exec(txn)
                .await?;
            publication.delete(txn).await?;
            Ok(())
        })
    }).await;

    match result {
        Ok(_) => Redirect::to(&state.urls.app()).into_response(),
        Err(e) => {
            eprintln!("[publications] delete failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
