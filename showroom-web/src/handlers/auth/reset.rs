use axum::{Extension, Form, extract::{Query, State}, response::{IntoResponse, Response}};
use boutique::{AppError, AppResult, UserContext, htmx, reset};
use serde::Deserialize;
use validator::Validate;

use crate::{mailer, state::AppState, views::{self, PageContext}};

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
}

pub async fn forgot_password_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> Response {
    views::auth::forgot_password(&PageContext::public(&ctx, state.urls.clone())).into_response()
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordForm>,
) -> AppResult {
    form.validate()?;

    match reset::request(&state.auth, &form.email).await {
        Ok(Some((user, token))) => {
            let reset_url = format!("{}/reset-password?token={}", state.urls.base(), token);
            mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls)
                .await
                .map_err(|e| AppError::internal(&format!("forgot-password email failed for {}", user.email), e))?;
        }
        Ok(None) => {}
        Err(e) => return Err(AppError::internal("forgot-password", e)),
    }

    Ok(views::auth::forgot_password_sent().into_response())
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: String,
}

pub async fn reset_password_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Query(params): Query<TokenQuery>,
) -> Response {
    let page_ctx = PageContext::public(&ctx, state.urls.clone());

    match reset::verify_token(&state.auth, &params.token).await {
        Ok(_) => views::auth::reset_password(&page_ctx, &params.token).into_response(),
        Err(_) => views::auth::reset_password_invalid(&page_ctx).into_response(),
    }
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordForm {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirm: String,
}

pub async fn reset_password(
    State(state): State<AppState>,
    Form(form): Form<ResetPasswordForm>,
) -> AppResult {
    form.validate()?;

    match reset::complete(&state.auth, &form.token, &form.password).await {
        Ok(_) => Ok(htmx::redirect(&format!("{}/login", state.urls.base()))),
        Err(reset::ResetError::InvalidToken) => Err(AppError::message("This link is invalid or has already been used")),
        Err(e) => Err(AppError::internal("reset-password", e)),
    }
}
