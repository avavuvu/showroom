use axum::{Router, routing::get};
use crate::{handlers, state::AppState};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/robots.txt", get(handlers::feeds::site_robots))
        .route("/sitemap.xml", get(handlers::sitemap::site))
}
