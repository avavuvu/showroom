use axum::{
    Router,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use crate::{handlers, routers::app::is_same_origin, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let comments = Router::new()
        .route("/{slug}/comments", post(handlers::comments::post_comment))
        .route("/{slug}/comments/sign-out", post(handlers::comments::sign_out))
        .route("/{slug}/comments/{id}/publish", post(handlers::comments::publish_comment))
        .route("/{slug}/comments/{id}", get(handlers::comments::show_comment).delete(handlers::comments::delete_comment))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_same_host));

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
        .route("/comments/session", get(handlers::comments::start_session))
        .route("/comments/confirm", get(handlers::comments::open_confirmation))
        .merge(comments)
        .with_state(state)
}

async fn require_same_host(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if request.method().is_safe() {
        return next.run(request).await;
    }

    let scheme = if state.urls.https { "https" } else { "http" };
    let host = request.headers().get(header::HOST).and_then(|value| value.to_str().ok());
    if host.is_some_and(|host| is_same_origin(request.headers(), &format!("{scheme}://{host}"))) {
        next.run(request).await
    } else {
        tracing::warn!(method = %request.method(), path = %request.uri().path(), "blocked a cross-origin comment request");
        StatusCode::FORBIDDEN.into_response()
    }
}
