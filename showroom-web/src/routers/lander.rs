use axum::{Router, routing::{get, post}};
use crate::{handlers, state::AppState};
use crate::{views::pages::about::about};

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .fallback(handlers::error404::lander_404)
        .route("/", get(handlers::home::index))
        .route("/about", get(handlers::passthrough(about)))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        .merge(super::sitemap::create_router());

    #[cfg(debug_assertions)]
    let router = router.route("/signup", get(handlers::auth::signup_page).post(handlers::auth::signup));

    router.with_state(state)
}
