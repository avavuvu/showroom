use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use axum::{
    Form,
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, sea_query::OnConflict};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::{extractors::AuthenticatedUser, jwt},
    mailer,
    models::{subscriber, user},
    state::AppState,
    views::{self, PageContext},
};

pub async fn get_settings(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    views::settings::index(&PageContext::from_user(&user, state.urls.clone()))
}

pub async fn request_password_change(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let token = match jwt::generate_password_reset(state.jwt_secret.as_bytes(), &user.id) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[password reset] token generation failed: {e}");
            return views::settings::change_password_error("Something went wrong, please try again");
        }
    };

    let reset_url = format!("{}/settings/change-password?token={}", state.urls.app(), token);

    match mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls).await {
        Ok(_) => views::settings::change_password_requested(),
        Err(e) => {
            eprintln!("[password reset] email failed for {}: {e}", user.email);
            views::settings::change_password_error(&format!("Failed to send email: {e}"))
        }
    }
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: String,
}

pub async fn get_change_password(
    AuthenticatedUser(_): AuthenticatedUser,
    Query(params): Query<TokenQuery>,
) -> Markup {
    views::settings::change_password_form(&params.token)
}

#[derive(Deserialize, Validate)]
pub struct ChangePasswordForm {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub password_confirm: String,
}

pub async fn post_change_password(
    State(state): State<AppState>,
    AuthenticatedUser(_): AuthenticatedUser,
    Form(form): Form<ChangePasswordForm>,
) -> Response {
    if form.password != form.password_confirm {
        return (StatusCode::BAD_REQUEST, "Passwords do not match").into_response();
    }

    if let Err(errors) = form.validate() {
        return (StatusCode::BAD_REQUEST, errors.to_string()).into_response();
    }

    let claims = match jwt::validate_password_reset(state.jwt_secret.as_bytes(), &form.token) {
        Ok(c) => c,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid or expired link").into_response(),
    };

    let user = match user::Entity::find_by_id(&claims.sub).one(&state.db).await {
        Ok(Some(u)) => u,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let mut active: user::ActiveModel = user.into();
    active.password = Set(hash);
    let _ = active.update(&state.db).await;

    views::settings::change_password_success().into_response()
}
