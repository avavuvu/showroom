use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{AppError, AppResult, UserContext, htmx, session::{self, LoginError}};
use serde::Deserialize;
use validator::Validate;

use crate::{state::AppState, views::{self, PageContext}};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

pub async fn login_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> Response {
    if ctx.is_authenticated() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::login(&PageContext::public(&ctx, state.urls.clone())).into_response()
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> AppResult {
    form.validate()?;

    match session::login(&state.auth, &form.email, &form.password).await {
        Ok((_, session)) => Ok((session, htmx::redirect(&state.urls.app())).into_response()),
        Err(LoginError::InvalidCredentials) => Err(AppError::message("Incorrect email or password")),
        Err(e) => Err(AppError::internal("login", e)),
    }
}
