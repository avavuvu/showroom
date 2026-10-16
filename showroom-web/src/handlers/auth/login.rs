use axum::{Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{AppResult, htmx, session};
use serde::Deserialize;
use validator::Validate;

use crate::{state::AppState, views::{self, PageContext, Viewer}};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

pub async fn login_page(
    State(state): State<AppState>,
    viewer: Viewer,
) -> Response {
    if viewer.is_some() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::login(&PageContext::public(viewer, state.urls.clone())).into_response()
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> AppResult {
    form.validate()?;

    let (_, session) = session::login(&state.auth, &form.email, &form.password).await?;
    Ok((session, htmx::redirect(&state.urls.app())).into_response())
}
