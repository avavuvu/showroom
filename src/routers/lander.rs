use axum::{Router, routing::{get, post}};

use tower_sessions::{MemoryStore, SessionManagerLayer};
use crate::{handlers, state::AppState};

pub fn create_router(
    state: AppState,
    session_layer: SessionManagerLayer<MemoryStore>,
) -> Router {
    Router::new()
        .route("/", get(handlers::home::index))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/signup", get(handlers::auth::signup_page).post(handlers::auth::signup))
        .route("/logout", post(handlers::auth::logout))
        .with_state(state)
        .layer(session_layer)
}
