use axum::{Router, routing::{get, post}};
use crate::views::auth::{login, signup};
use crate::{handlers, state::AppState};
use crate::{views::pages::about::about};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .fallback(handlers::error404::lander_404)
        .route("/", get(handlers::home::index))
        .route("/about", get(handlers::passthrough(about)))
        .route("/login", get(handlers::passthrough(login)).post(handlers::auth::login))
        .route("/signup", get(handlers::passthrough(signup)).post(handlers::auth::signup))
        .route("/logout", post(handlers::auth::logout))
        .with_state(state)
}
