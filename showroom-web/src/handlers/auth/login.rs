use axum::{Form, extract::{Query, State}, http::HeaderMap, response::{IntoResponse, Redirect, Response}};
use boutique::{AppError, AppResult, htmx, session};
use serde::Deserialize;
use validator::Validate;

use crate::{
    handlers,
    services::rate_limit::{Limit, client_ip},
    state::AppState,
    views::{self, PageContext, Viewer},
};

const PER_ADDRESS: Limit = Limit::per_quarter_hour(10);
const PER_CLIENT: Limit = Limit::per_quarter_hour(30);

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    pub next: Option<String>,
}

#[derive(Deserialize)]
pub struct NextQuery {
    pub next: Option<String>,
}

fn safe_next<'a>(state: &AppState, next: Option<&'a str>) -> Option<&'a str> {
    let allowed = format!("{}/visit?", state.urls.app());
    next.filter(|next| next.starts_with(&allowed) && handlers::is_safe_location(next))
}

pub async fn login_page(
    State(state): State<AppState>,
    Query(query): Query<NextQuery>,
    viewer: Viewer,
) -> Response {
    let next = safe_next(&state, query.next.as_deref());
    if viewer.is_some() {
        return Redirect::to(next.unwrap_or(&state.urls.app())).into_response();
    }

    views::auth::login(&PageContext::public(viewer, state.urls.clone()), next).into_response()
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(mut form): Form<LoginForm>,
) -> AppResult {
    form.email = form.email.trim().to_string();
    form.validate()?;

    let mut checks = vec![(format!("login:{}", form.email.to_lowercase()), PER_ADDRESS)];
    if let Some(ip) = client_ip(&headers) {
        checks.push((format!("login:{ip}"), PER_CLIENT));
    }
    if !state.limits.allow_all(&checks) {
        return Err(AppError::message("Too many login attempts. Please wait a few minutes and try again."));
    }

    let (_, session) = session::login(&state.auth, &form.email, &form.password).await?;
    let next = safe_next(&state, form.next.as_deref()).map_or_else(|| state.urls.app(), str::to_string);
    Ok((session, htmx::redirect(&next)).into_response())
}
