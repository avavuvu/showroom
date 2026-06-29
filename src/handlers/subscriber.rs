use axum::{
    Form,
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    mailer,
    models::{
        subscriber::{self, Entity as Subscriber},
        user::{self, Entity as User},
    },
    services::subdomain::UsernameSubdomain,
    state::AppState,
    views,
};

#[derive(Deserialize)]
pub struct SubscribeForm {
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
) -> Result<Markup, StatusCode> {
    let user = User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let token = nanoid!(21);

    let result_of_insert = subscriber::ActiveModel {
        token: Set(token.clone()),
        user_id: Set(user.id),
        name: Set(form.name.clone()),
        email: Set(form.email.clone()),
        is_confirmed: Set(false),
        created_at: Set(chrono::Utc::now().fixed_offset()),
    }
    .insert(&state.db)
    .await;

    match result_of_insert {
        Err(DbErr::RecordNotInserted) | Err(DbErr::Exec(_)) => {
            return Ok(views::subscriber::subscribe_exists());
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(_) => {}
    }

    mailer::send_confirmation(
        &state.ses,
        &form.email,
        form.name.as_deref(),
        &token,
        &handle,
        &state.urls,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(views::subscriber::subscribe_success())
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
