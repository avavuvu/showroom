use axum::{Router, routing::{get, post}};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .fallback(handlers::error404::user_404)
        .route("/", get(handlers::user::profile))
        .route("/{slug}", get(handlers::user::newsletter))
        .route("/newsletters", get(handlers::user::get_newsletters))
        .route("/api/newsletters", get(handlers::api::get_newsletters))
        .route("/api/newsletters/{slug}", get(handlers::api::get_newsletter))
        .route("/subscribe", post(handlers::subscriber::subscribe))
        .route("/confirm", get(handlers::subscriber::confirm))
        .route("/unsubscribe", get(handlers::subscriber::unsubscribe).post(handlers::subscriber::unsubscribe))
        .with_state(state)
}
