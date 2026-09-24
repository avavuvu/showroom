use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{UserContext, htmx, session::{self, LoginError}};
use serde::Deserialize;
use validator::Validate;

use super::something_went_wrong;
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
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    match session::login(&state.auth, &form.email, &form.password).await {
        Ok((_, session)) => (session, htmx::redirect(&state.urls.app())).into_response(),
        Err(LoginError::InvalidCredentials) => {
            htmx::fragments::error("Incorrect email or password").into_response()
        }
        Err(e) => {
            eprintln!("[login] {e:?}");
            something_went_wrong()
        }
    }
}
