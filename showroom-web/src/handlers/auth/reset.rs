use axum::{Extension, Form, extract::{Query, State}, response::{IntoResponse, Response}};
use boutique::{UserContext, htmx, reset};
use serde::Deserialize;
use validator::Validate;

use super::something_went_wrong;
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
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    match reset::request(&state.auth, &form.email).await {
        Ok(Some((user, token))) => {
            let reset_url = format!("{}/reset-password?token={}", state.urls.base(), token);
            if let Err(e) = mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls).await {
                eprintln!("[forgot-password] email failed for {}: {e}", user.email);
                return something_went_wrong();
            }
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("[forgot-password] {e:?}");
            return something_went_wrong();
        }
    }

    views::auth::forgot_password_sent().into_response()
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
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    match reset::complete(&state.auth, &form.token, &form.password).await {
        Ok(_) => htmx::redirect(&format!("{}/login", state.urls.base())),
        Err(reset::ResetError::InvalidToken) => {
            htmx::fragments::error("This link is invalid or has already been used").into_response()
        }
        Err(e) => {
            eprintln!("[reset-password] {e:?}");
            something_went_wrong()
        }
    }
}
