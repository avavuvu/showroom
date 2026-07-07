use axum::{Router, routing::get};
use crate::{handlers, state::AppState};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/sitemap.xml", get(handlers::sitemap::index))
        .route("/sitemap-pages.xml", get(handlers::sitemap::pages))
        .route("/sitemap-users.xml", get(handlers::sitemap::users))
        .route("/sitemap-newsletters.xml", get(handlers::sitemap::newsletters))
}
