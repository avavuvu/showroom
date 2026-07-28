use axum::{extract::State, http::StatusCode, Extension};
use maud::Markup;

use crate::{
    auth::{context::UserContext, extractors::AuthenticatedUser},
    services::subdomain::UsernameSubdomain,
    state::AppState,
    views::{pages::error404, PageContext},
};

pub async fn lander_404(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(&ctx, state.urls.clone());
    (StatusCode::NOT_FOUND, error404::lander_404(&page_ctx))
}

pub async fn app_404(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::from_user(&user, state.urls.clone());
    (StatusCode::NOT_FOUND, error404::app_404(&page_ctx))
}

pub async fn user_404(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Extension(ctx): Extension<UserContext>,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(&ctx, state.urls.clone())
        .with_page_owner(&handle);
    (StatusCode::NOT_FOUND, error404::user_404(&page_ctx))
}
