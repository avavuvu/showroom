use axum::{
    Form, extract::State, http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;

use boutique::{AppError, AppResult, AuthenticatedUser};
use crate::{
    services::publication::OwnedPublication,
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        publication_name::{Article, NameCase, Noun, Possessive, PublicationName},
        subscriber::{self, Entity as Subscriber},
        user,
    },
    state::AppState,
    theme::{Color, Font, Fonts, Layout, Overrides, Theme, Trim},
    components::NameCases,
    views::{self, PageContext},
};

#[derive(Deserialize)]
pub struct NameForm {
    pub article: Article,
    pub label: String,
    pub possessive: Possessive,
    pub noun: Noun,
    pub name_case: NameCase,
}

impl NameForm {
    fn name(&self) -> PublicationName {
        let label = self.label.split_whitespace().collect::<Vec<_>>().join(" ");
        PublicationName { article: self.article, label, possessive: self.possessive, noun: self.noun }
    }
}

pub async fn preview_name(_: AuthenticatedUser<user::Model>, Form(form): Form<NameForm>) -> Markup {
    let name = form.name().render(form.name_case);
    boutique::html! { (name) }
}

async fn account_context(user: &user::Model, state: &AppState) -> Result<PageContext, sea_orm::DbErr> {
    let publications = publication::for_owner(&user.id, &state.db).await?;
    Ok(PageContext::from_user(user, state.urls.clone()).with_publications(publications))
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
    #[serde(flatten)]
    pub name: NameForm,
}

pub async fn new_form(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> AppResult<Markup> {
    Ok(views::dashboard::publications::new_form(&account_context(&user, &state).await?, "", &PublicationName::default(), NameCase::default(), None))
}

pub async fn create(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
    Form(form): Form<NewPublicationForm>,
) -> Response {
    let ctx = match account_context(&user, &state).await {
        Ok(ctx) => ctx,
        Err(error) => return AppError::from(error).into_response(),
    };
    let slug = form.slug.trim().to_lowercase();
    let name = form.name.name();
    let case = form.name.name_case;

    let retry = |message: &str| views::dashboard::publications::new_form(&ctx, &slug, &name, case, Some(message)).into_response();

    if let Err(message) = name.validate() {
        return retry(message);
    }

    if let Err(message) = publication::validate_slug(&slug) {
        return retry(message);
    }

    let taken = match Publication::find().filter(publication::Column::Slug.eq(&slug)).one(&state.db).await {
        Ok(existing) => existing.is_some(),
        Err(error) => {
            tracing::error!(%error, "checking a publication address failed");
            return retry("Something went wrong, please try again");
        }
    };

    if taken {
        return retry("This address is already taken");
    }

    let now = chrono::Utc::now().fixed_offset();
    let new_publication = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: Set(user.id.clone()),
        slug: Set(slug.clone()),
        name: Set(name.render(case)),
        name_case: Set(case),
        description: Set(None),
        theme: Set(None),
        image: Set(publication::random_picture()),
        banner: Set(None),
        pictures: Set(Default::default()),
        is_default: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    match new_publication.insert(&state.db).await {
        Ok(_) => Redirect::to(&state.urls.dashboard(&slug)).into_response(),
        Err(error) => {
            tracing::error!(%error, "creating a publication failed");
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
    #[serde(flatten)]
    pub name: NameForm,
    pub description: Option<String>,
}

pub async fn update_settings(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Form(form): Form<SettingsForm>,
) -> AppResult {
    let description = form.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());
    let name = form.name.name();
    if let Err(message) = name.validate() {
        return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some(message)).into_response());
    }

    let slug = owned.publication.slug.clone();
    let mut active: publication::ActiveModel = owned.publication.into();
    active.name = Set(name.render(form.name.name_case));
    active.name_case = Set(form.name.name_case);
    active.description = Set(description);
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
    pub trim: Option<String>,
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
            trim: match &self.trim {
                Some(key) => Trim::parse(key)?,
                None => Trim::default(),
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
        return Ok(views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some("Colors must be in #rrggbb format, and fonts, layouts and trims must be from the list")).into_response());
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
