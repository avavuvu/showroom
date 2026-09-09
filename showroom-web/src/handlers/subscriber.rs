use axum::{
    Form, extract::{Query, State}, http::StatusCode, response::{Redirect, Response, IntoResponse},
};
use validator::Validate;
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    htmx, mailer, models::{publication, subscriber::{self, Entity as Subscriber}},
    services::subdomain::CurrentPublication, state::AppState, views,
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
    CurrentPublication(publication): CurrentPublication,
    Form(form): Form<SubscribeForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    insert_or_resend(&state, &publication, &form).await
}

pub async fn confirm(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Query(params): Query<TokenQuery>,
) -> Result<Redirect, StatusCode> {
    let subscriber = Subscriber::find_by_id(&params.token)
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
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
        state.urls.publication(&publication.slug)
    )))
}

pub async fn unsubscribe(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Query(params): Query<TokenQuery>,
) -> Result<Markup, StatusCode> {
    Subscriber::delete_many()
        .filter(subscriber::Column::Token.eq(&params.token))
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

        Err(DbErr::RecordNotInserted) |
        Err(DbErr::Exec(_)) |
        Err(DbErr::Query(_)) => resend_if_unconfirmed(state, &form.email, publication).await,

        Err(e) => views::subscriber::subscribe_error(&format!("Something went wrong, please try again: {e}")).into_response(),
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
        _ => views::subscriber::subscribe_exists().into_response(),
    }
}

async fn send_confirmation(state: &AppState, email: &str, name: Option<&str>, token: &str, publication: &publication::Model) -> Response {
    let confirmation_response = mailer::send_confirmation(&state.ses, email, name, token, publication, &state.urls)
        .await;

    if let Err(e) = confirmation_response {
        return views::subscriber::subscribe_error(&format!("Failed to send confirmation email, please try again, {e}")).into_response();
    }

    views::subscriber::subscribe_success().into_response()
}
