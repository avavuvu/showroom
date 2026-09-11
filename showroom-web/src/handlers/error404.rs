use axum::{extract::State, http::StatusCode, Extension};
use maud::Markup;

use boutique::{AuthenticatedUser, UserContext};
use crate::{
    services::subdomain::CurrentPublication,
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

pub async fn publication_404(
    State(state): State<AppState>,
    publication: Result<CurrentPublication, StatusCode>,
    Extension(ctx): Extension<UserContext>,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(&ctx, state.urls.clone());

    let page = match publication {
        Ok(CurrentPublication(publication)) => error404::publication_404(&page_ctx.with_publication(publication)),
        Err(_) => error404::lander_404(&page_ctx),
    };

    (StatusCode::NOT_FOUND, page)
}
