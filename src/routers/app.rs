use axum::{middleware, Router, routing::get};
use crate::{auth::middleware::required_auth, handlers::dashboard::*, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/newsletters", get(get_newsletters))
        .route("/edit/{id}", get(get_edit))
        .route("/preview/{slug}", get(get_preview))
        .route("/json/{id}", get(get_edit_json).put(put_edit_json))
        .layer(middleware::from_fn_with_state(state.clone(), required_auth))
        .with_state(state)
}
