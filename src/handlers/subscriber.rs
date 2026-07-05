use axum::{
    Form, extract::{Query, State}, http::StatusCode, response::{Redirect, Response, IntoResponse},
};
use validator::Validate;
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    htmx, mailer, models::{
        subscriber::{self, Entity as Subscriber},
        user::{self, Entity as User},
    }, services::subdomain::UsernameSubdomain, state::AppState, views,
};

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
    UsernameSubdomain(handle): UsernameSubdomain,
    Form(form): Form<SubscribeForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    let user = match User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
    {
        Ok(Some(u)) => u,
        _ => return views::subscriber::subscribe_error("Something went wrong, please try again").into_response(),
    };

    insert_or_resend(&state, &user.id, &form, &handle).await
}

pub async fn confirm(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Query(params): Query<TokenQuery>,
) -> Result<Redirect, StatusCode> {
    let subscriber = Subscriber::find_by_id(&params.token)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut active: subscriber::ActiveModel = subscriber.into();
    active.is_confirmed = Set(true);
    active.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!(
        "{}?from=confirmation",
        state.urls.user(&handle)
    )))
}

pub async fn unsubscribe(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Query(params): Query<TokenQuery>,
) -> Result<Markup, StatusCode> {
    Subscriber::delete_by_id(&params.token)
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(views::subscriber::unsubscribed(&handle))
}

async fn insert_or_resend(state: &AppState, user_id: &str, form: &SubscribeForm, handle: &str) -> Response {
    let token = nanoid!(21);

    let subscriber_response = (subscriber::ActiveModel {
        token: Set(token.clone()),
        user_id: Set(user_id.to_string()),
        name: Set(form.name.clone()),
        email: Set(form.email.clone()),
        is_confirmed: Set(false),
        created_at: Set(chrono::Utc::now().fixed_offset())
    })
        .insert(&state.db)
        .await;

    match subscriber_response {
        Ok(_) => send_confirmation(state, &form.email, form.name.as_deref(), &token, handle).await,

        Err(DbErr::RecordNotInserted) |
        Err(DbErr::Exec(_)) |
        Err(DbErr::Query(_)) => resend_if_unconfirmed(state, &form.email, user_id, handle).await,

        Err(e) => views::subscriber::subscribe_error(&format!("Something went wrong, please try again: {e}")).into_response(),
    }
}

async fn resend_if_unconfirmed(state: &AppState, email: &str, user_id: &str, handle: &str) -> Response {
    let existing = Subscriber::find()
        .filter(subscriber::Column::Email.eq(email))
        .filter(subscriber::Column::UserId.eq(user_id))
        .one(&state.db)
        .await;

    match existing {
        Ok(Some(sub)) if !sub.is_confirmed => {
            send_confirmation(state, &sub.email, sub.name.as_deref(), &sub.token, handle).await
        }
        _ => views::subscriber::subscribe_exists().into_response(),
    }
}

async fn send_confirmation(state: &AppState, email: &str, name: Option<&str>, token: &str, handle: &str) -> Response {
    let confirmation_response = mailer::send_confirmation(&state.ses, email, name, token, handle, &state.urls)
        .await;

    if let Err(e) = confirmation_response {
        return views::subscriber::subscribe_error("Failed to send confirmation email, please try again").into_response();
    }

    views::subscriber::subscribe_success().into_response()
}
