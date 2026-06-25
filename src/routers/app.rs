use axum::{Router, routing::get};

use tower_sessions::{MemoryStore, SessionManagerLayer};
use crate::{handlers, state::AppState};

pub fn create_router(
    state: AppState,
    session_layer: SessionManagerLayer<MemoryStore>,
) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard::index))
        .with_state(state)
        .layer(session_layer)
}
