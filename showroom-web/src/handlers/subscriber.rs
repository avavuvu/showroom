use axum::{
    Form, extract::{Query, State}, http::HeaderMap, response::{Redirect, Response, IntoResponse},
};
use boutique::{AppError, AppResult};
use validator::Validate;
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, SqlErr};
use serde::Deserialize;

use crate::{
    mailer, models::{publication, subscriber::{self, Entity as Subscriber}},
    services::{rate_limit::{Limit, client_ip}, subdomain::CurrentPublication}, state::AppState, views,
};

const PER_ADDRESS: Limit = Limit::per_hour(3);
const PER_CLIENT: Limit = Limit::per_hour(20);
const TOO_MANY: &str = "Too many attempts. Please try again later.";
const SOMETHING_WENT_WRONG: &str = "Something went wrong. Please try again.";

#[derive(Deserialize, Validate)]
pub struct SubscribeForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: String,
}

pub async fn subscribe(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    headers: HeaderMap,
    Form(mut form): Form<SubscribeForm>,
) -> AppResult {
    form.email = form.email.trim().to_lowercase();
    form.name = form.name.map(|name| name.trim().to_string()).filter(|name| !name.is_empty());
    form.validate()?;

    let mut checks = vec![(format!("subscribe:{}:{}", publication.id, form.email), PER_ADDRESS)];
    if let Some(ip) = client_ip(&headers) {
        checks.push((format!("subscribe:{ip}"), PER_CLIENT));
    }
    if !state.limits.allow_all(&checks) {
        return Ok(views::subscriber::subscribe_error(TOO_MANY).into_response());
    }

    Ok(insert_or_resend(&state, &publication, &form).await)
}

pub async fn confirm_page(CurrentPublication(publication): CurrentPublication) -> Markup {
    views::subscriber::confirm_subscription(&publication.name)
}

pub async fn unsubscribe_page(CurrentPublication(publication): CurrentPublication) -> Markup {
    views::subscriber::confirm_unsubscribe(&publication.name)
}

pub async fn confirm(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Query(params): Query<TokenQuery>,
) -> AppResult<Redirect> {
    let subscriber = Subscriber::find_by_id(&params.token)
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut active: subscriber::ActiveModel = subscriber.into();
    active.is_confirmed = Set(true);
    active.update(&state.db).await?;

    Ok(Redirect::to(&format!(
        "{}?from=confirmation",
        state.urls.publication(&publication.slug)
    )))
}

pub async fn unsubscribe(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Query(params): Query<TokenQuery>,
) -> AppResult<Markup> {
    Subscriber::delete_many()
        .filter(subscriber::Column::Token.eq(&params.token))
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
        .exec(&state.db)
        .await?;

    Ok(views::subscriber::unsubscribed(&publication.name))
}

async fn insert_or_resend(state: &AppState, publication: &publication::Model, form: &SubscribeForm) -> Response {
    let token = nanoid!(21);

    let subscriber_response = (subscriber::ActiveModel {
        token: Set(token.clone()),
        publication_id: Set(publication.id.clone()),
        name: Set(form.name.clone()),
        email: Set(form.email.clone()),
        is_confirmed: Set(false),
        created_at: Set(chrono::Utc::now().fixed_offset())
    })
        .insert(&state.db)
        .await;

    match subscriber_response {
        Ok(_) => send_confirmation(state, &form.email, form.name.as_deref(), &token, publication).await,
        Err(error) if matches!(error.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
            resend_if_unconfirmed(state, &form.email, publication).await
        }
        Err(error) => {
            tracing::error!(%error, publication = %publication.id, "subscribing failed");
            views::subscriber::subscribe_error(SOMETHING_WENT_WRONG).into_response()
        }
    }
}

async fn resend_if_unconfirmed(state: &AppState, email: &str, publication: &publication::Model) -> Response {
    let existing = Subscriber::find()
        .filter(subscriber::Column::Email.eq(email))
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await;

    match existing {
        Ok(Some(sub)) if !sub.is_confirmed => {
            send_confirmation(state, &sub.email, sub.name.as_deref(), &sub.token, publication).await
        }
        Ok(_) => views::subscriber::subscribe_exists().into_response(),
        Err(error) => {
            tracing::error!(%error, publication = %publication.id, "looking up an existing subscriber failed");
            views::subscriber::subscribe_error(SOMETHING_WENT_WRONG).into_response()
        }
    }
}

async fn send_confirmation(state: &AppState, email: &str, name: Option<&str>, token: &str, publication: &publication::Model) -> Response {
    let confirmation_response = mailer::send_confirmation(&state.ses, email, name, token, publication, &state.urls)
        .await;

    if let Err(error) = confirmation_response {
        tracing::error!(%error, publication = %publication.id, "sending a confirmation email failed");
        return views::subscriber::subscribe_error("The confirmation email couldn't be sent. Please try again.").into_response();
    }

    views::subscriber::subscribe_success().into_response()
}
