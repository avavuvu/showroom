use axum::{extract::State, http::StatusCode};
use maud::Markup;

use boutique::{AppError, AuthenticatedUser};
use crate::{
    models::user,
    services::subdomain::CurrentPublication,
    state::AppState,
    views::{pages::error404, PageContext, Viewer},
};

pub async fn lander_404(
    State(state): State<AppState>,
    viewer: Viewer,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(viewer, state.urls.clone());
    (StatusCode::NOT_FOUND, error404::lander_404(&page_ctx))
}

pub async fn app_404(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::from_user(&user, state.urls.clone());
    (StatusCode::NOT_FOUND, error404::app_404(&page_ctx))
}

pub async fn publication_404(
    State(state): State<AppState>,
    publication: Result<CurrentPublication, AppError>,
    viewer: Viewer,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(viewer, state.urls.clone());

    let page = match publication {
        Ok(CurrentPublication(publication)) => error404::publication_404(&page_ctx.with_publication(publication)),
        Err(_) => error404::lander_404(&page_ctx),
    };

    (StatusCode::NOT_FOUND, page)
}
