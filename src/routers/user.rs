use axum::{Router, routing::{get, post}};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::user::profile))
        .route("/{slug}", get(handlers::user::newsletter))
        .route("/subscribe", post(handlers::subscriber::subscribe))
        .route("/confirm", get(handlers::subscriber::confirm))
        .route("/unsubscribe", get(handlers::subscriber::unsubscribe))
        .with_state(state)
}
