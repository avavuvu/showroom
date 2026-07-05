use axum::{middleware, Router, routing::{get}};
use crate::{auth::middleware::required_auth, handlers::{dashboard::*, error404::app_404}, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .fallback(app_404)
        .route("/", get(index))
        .route("/newsletters", get(get_newsletters).post(post_newsletters))
        .route("/edit/{id}", get(get_edit))
        .route("/send/{id}", get(get_send).post(post_send))
        .route("/json/{id}", get(get_edit_json).put(put_edit_json))
        .layer(middleware::from_fn_with_state(state.clone(), required_auth))
        .with_state(state)
}
