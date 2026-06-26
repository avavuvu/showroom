use axum::{Router, routing::{get, post}};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::home::index))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/signup", get(handlers::auth::signup_page).post(handlers::auth::signup))
        .route("/logout", post(handlers::auth::logout))
        .with_state(state)
}
