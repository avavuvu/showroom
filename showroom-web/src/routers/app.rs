use axum::{
    Router,
    extract::{DefaultBodyLimit, Request, State},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
};
use crate::{handlers::{dashboard::*, error404::app_404}, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let publication = Router::new()
        .route("/", get(index))
        .route("/newsletters", post(post_newsletters))
        .route("/newsletters/{id}", delete(delete_newsletter))
        .route("/edit/{id}", get(get_edit))
        .route("/send/{id}", get(get_send).post(post_send))
        .route("/send/{id}/progress", get(get_progress))
        .route("/send/{id}/retry", post(post_retry))
        .route("/edit/{id}/review", get(reviews::new_review_page).post(reviews::create_review))
        .route("/edit/{id}/reviews", get(reviews::newsletter_reviews))
        .route("/reviews/{review}/links/{link}", get(reviews::link_results))
        .route("/reviews/{review}/links/{link}/apply", post(reviews::apply_link))
        .route("/reviews/{review}/links/{link}/restore", post(reviews::restore_link))
        .route("/reviews/{review}/links/{link}/delete", post(reviews::delete_link))
        .route("/review/{token}", get(reviews::get_review_page).put(reviews::put_review_json))
        .route("/review/{token}/send", post(reviews::post_review_send))
        .route("/subscribers", get(subscribers::get_subscribers))
        .route("/subscribers/import", post(subscribers::import_subscribers))

        .route("/settings", get(publications::get_settings).post(publications::update_settings))
        .route("/settings/style", post(publications::update_style))
        .route("/settings/picture", post(images::update_picture).layer(DefaultBodyLimit::max(images::MAX_IMAGE_BYTES + 64 * 1024)))
        .route("/settings/banner", post(images::update_banner).layer(DefaultBodyLimit::max(images::MAX_IMAGE_BYTES + 64 * 1024)))
        .route("/settings/banner/remove", post(images::remove_banner))
        .route("/settings/style/preview", post(publications::preview_style))
        .route("/delete", post(publications::delete));

    Router::new()
        .fallback(app_404)
        .route("/", get(publications::index))
        .route("/robots.txt", get(crate::handlers::feeds::app_robots))
        .route("/new", get(publications::new_form).post(publications::create))
        .route("/name/preview", post(publications::preview_name))
        .route("/settings", get(settings::get_settings))
        .route("/settings/change-password/request", post(settings::request_password_change))
        .route("/json/{id}", put(put_edit_json))
        .route("/images/sign", get(images::sign_upload))
        .route("/music/resolve", post(music::resolve))
        .route("/music/color", post(music::color))
        .route("/youtube/lookup", post(youtube::lookup))
        .nest("/{slug}", publication)
        .layer(middleware::from_fn_with_state(state.clone(), require_same_origin))
        .with_state(state)
}

async fn require_same_origin(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if request.method().is_safe() {
        return next.run(request).await;
    }

    if is_same_origin(request.headers(), &state.urls.app()) {
        next.run(request).await
    } else {
        tracing::warn!(method = %request.method(), path = %request.uri().path(), "blocked a cross-origin request");
        StatusCode::FORBIDDEN.into_response()
    }
}

fn is_same_origin(headers: &header::HeaderMap, app: &str) -> bool {
    let value = |name| headers.get(name).and_then(|value: &header::HeaderValue| value.to_str().ok());
    match value(header::ORIGIN) {
        Some(origin) => origin == app,
        None => value(header::REFERER).is_some_and(|referer| referer == app || referer.starts_with(&format!("{app}/"))),
    }
}
