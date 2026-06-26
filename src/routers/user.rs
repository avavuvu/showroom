use axum::{Router, routing::get};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::user::profile))
        .with_state(state)
}
