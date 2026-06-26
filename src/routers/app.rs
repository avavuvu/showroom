use axum::{middleware, Router, routing::get};
use crate::{auth::middleware::required_auth, handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard::index))
        .route("/newsletters", get(handlers::dashboard::get_newsletters))
        .route("/edit/{slug}", get(handlers::dashboard::get_edit))
        .route("/preview/{slug}", get(handlers::dashboard::get_preview))
        .layer(middleware::from_fn_with_state(state.clone(), required_auth))
        .with_state(state)
}
