use axum::{
    Form, extract::State, http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;

use boutique::{AppError, AppResult, AuthenticatedUser};
use crate::{
    renderer::greeting,
    services::publication::OwnedPublication,
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        subscriber::{self, Entity as Subscriber},
        user,
    },
    state::AppState,
    theme::{Color, Font, Fonts, Layout, Overrides, Theme},
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
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> AppResult<Redirect> {
    let publications = publication::for_owner(&user.id, &state.db)
        .await
        .map_err(|e| AppError::internal(&format!("failed to load publications for {}", user.id), e))?;

    match publications.iter().find(|p| p.is_default).or(publications.first()) {
        Some(room) => Ok(Redirect::to(&state.urls.dashboard(&room.slug))),
        None => Ok(Redirect::to(&format!("{}/new", state.urls.app()))),
    }
}

#[derive(Deserialize)]
pub struct NewPublicationForm {
    pub slug: String,
    pub name: String,
}

pub async fn new_form(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> Markup {
    views::dashboard::publications::new_form(&account_context(&user, &state).await, "", "", None)
}

pub async fn create(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
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
        greeting: Set(greeting::DEFAULT.to_string()),
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
    #[serde(default)]
    pub greeting: String,
}

pub async fn update_settings(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Form(form): Form<SettingsForm>,
) -> AppResult {
    let name = form.name.trim().to_string();
    let description = form.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());

    if name.is_empty() {
        return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some("A name is required")).into_response());
    }

    let greeting = match greeting::normalize(form.greeting.trim()) {
        Ok(greeting) => greeting,
        Err(message) => return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some(message)).into_response()),
    };

    if greeting.chars().count() > greeting::MAX_LEN {
        return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some("The greeting is too long")).into_response());
    }

    let slug = owned.publication.slug.clone();
    let mut active: publication::ActiveModel = owned.publication.into();
    active.name = Set(name);
    active.description = Set(description);
    active.greeting = Set(greeting);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    active.update(&state.db).await?;

    Ok(Redirect::to(&format!("{}/settings", state.urls.dashboard(&slug))).into_response())
}

#[derive(Deserialize)]
pub struct StyleForm {
    pub ink: String,
    pub paper: String,
    pub brand: String,
    #[serde(default)]
    pub link: String,
    pub link_custom: Option<String>,
    #[serde(default)]
    pub muted: String,
    pub muted_custom: Option<String>,
    #[serde(default, rename = "on-brand")]
    pub on_brand: String,
    #[serde(rename = "on-brand_custom")]
    pub on_brand_custom: Option<String>,
    pub font_title: Option<String>,
    pub font_body: Option<String>,
    pub layout: Option<String>,
}

fn font(key: &Option<String>, default: Font) -> Option<Font> {
    match key {
        Some(key) => Font::from_key(key),
        None => Some(default),
    }
}

fn override_color(custom: &Option<String>, value: &str) -> Option<Option<Color>> {
    match custom {
        Some(_) => Color::parse(value).map(Some),
        None => Some(None),
    }
}

impl StyleForm {
    fn theme(&self) -> Option<Theme> {
        Some(Theme {
            ink: Color::parse(&self.ink)?,
            paper: Color::parse(&self.paper)?,
            brand: Color::parse(&self.brand)?,
            overrides: Overrides {
                link: override_color(&self.link_custom, &self.link)?,
                muted: override_color(&self.muted_custom, &self.muted)?,
                on_brand: override_color(&self.on_brand_custom, &self.on_brand)?,
            },
            fonts: Fonts {
                title: font(&self.font_title, Fonts::default().title)?,
                body: font(&self.font_body, Fonts::default().body)?,
            },
            layout: match &self.layout {
                Some(key) => Layout::from_key(key)?,
                None => Layout::default(),
            },
        })
    }
}

pub async fn update_style(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Form(form): Form<StyleForm>,
) -> AppResult {
    let Some(theme) = form.theme() else {
        return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some("Colors must be in #rrggbb format, and fonts and layouts must be from the list")).into_response());
    };

    let slug = owned.publication.slug.clone();
    let mut active: publication::ActiveModel = owned.publication.into();
    active.theme = Set(Some(theme));
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    active.update(&state.db).await?;

    Ok(Redirect::to(&format!("{}/settings", state.urls.dashboard(&slug))).into_response())
}

pub async fn preview_style(_: OwnedPublication, Form(form): Form<StyleForm>) -> Response {
    match form.theme() {
        Some(theme) => ([(header::CONTENT_TYPE, "text/css")], theme.css()).into_response(),
        None => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
) -> AppResult<Redirect> {
    if publication.is_default {
        return Err(AppError::Status(StatusCode::FORBIDDEN));
    }

    state.db.transaction::<_, (), sea_orm::DbErr>(|txn| {
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
    }).await.map_err(|e| AppError::internal("publications delete", e))?;

    Ok(Redirect::to(&state.urls.app()))
}
