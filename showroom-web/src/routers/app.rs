use axum::{middleware, Router, routing::{delete, get, post}};
use boutique::middleware::required_auth;
use crate::{handlers::{dashboard::*, error404::app_404}, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let publication = Router::new()
        .route("/", get(index))
        .route("/newsletters", post(post_newsletters))
        .route("/newsletters/{id}", delete(delete_newsletter))
        .route("/edit/{id}", get(get_edit))
        .route("/send/{id}", get(get_send).post(post_send))
        .route("/subscribers", get(subscribers::get_subscribers))
        .route("/subscribers/import", post(subscribers::import_subscribers))

        .route("/settings", get(publications::get_settings).post(publications::update_settings))
        .route("/delete", post(publications::delete));

    Router::new()
        .fallback(app_404)
        .route("/", get(publications::index))
        .route("/new", get(publications::new_form).post(publications::create))
        .route("/settings", get(settings::get_settings))
        .route("/settings/change-password/request", post(settings::request_password_change))
        .route("/json/{id}", get(get_edit_json).put(put_edit_json))
        .route("/images/sign", get(images::sign_upload))
        .nest("/{slug}", publication)
        .layer(middleware::from_fn_with_state(state.auth.clone(), required_auth))
        .with_state(state)
}
