use axum::{Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{AppError, AppResult, htmx, session};

use crate::{
    services::account::{self, AccountError, NewAccount},
    state::AppState,
    views::{self, PageContext, Viewer},
};

#[cfg(debug_assertions)]
pub async fn signup_page(
    State(state): State<AppState>,
    viewer: Viewer,
) -> Response {
    if viewer.is_some() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::signup(&PageContext::public(viewer, state.urls.clone())).into_response()
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form): Form<NewAccount>,
) -> AppResult {
    let user = account::create(&state.db, &form).await.map_err(|e| match e {
        AccountError::Invalid(errors) => AppError::Validation(errors),
        AccountError::Fields(fields) => AppError::Fields(fields),
        AccountError::Hash(e) => AppError::internal("signup password hash", e),
        AccountError::Database(e) => AppError::internal("signup", e),
    })?;

    let session = session::issue(&state.auth, &user)
        .await
        .map_err(|e| AppError::internal("signup session", e))?;

    Ok((session, htmx::redirect(&state.urls.app())).into_response())
}
