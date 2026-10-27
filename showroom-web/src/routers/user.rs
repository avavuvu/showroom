use axum::{Router, routing::{get, post}};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .fallback(handlers::error404::publication_404)
        .route("/", get(handlers::user::profile))
        .route("/{slug}", get(handlers::user::newsletter))
        .route("/feed", get(handlers::feeds::rss))
        .route("/llms.txt", get(handlers::feeds::llms))
        .route("/robots.txt", get(handlers::feeds::publication_robots))
        .route("/sitemap.xml", get(handlers::sitemap::publication))
        .route("/api/newsletters", get(handlers::api::get_newsletters))
        .route("/api/newsletters/{slug}", get(handlers::api::get_newsletter))
        .route("/subscribe", post(handlers::subscriber::subscribe))
        .route("/confirm", get(handlers::subscriber::confirm_page).post(handlers::subscriber::confirm))
        .route("/unsubscribe", get(handlers::subscriber::unsubscribe_page).post(handlers::subscriber::unsubscribe))
        .with_state(state)
}
