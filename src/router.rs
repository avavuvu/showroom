use axum::middleware;
use sea_orm::DatabaseConnection;
use axum::http::{HeaderValue, header};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;
use crate::{auth::middleware::base, routers::*, state::{AppState, Urls}, services::subdomain::SubdomainRouter};

pub fn create_service(db: DatabaseConnection, domain: &str, port: &str, jwt_secret: String, is_production: bool) -> axum::Router {
    let state = AppState { db, urls: Urls::new(domain, port, is_production), jwt_secret };

    let static_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .service(ServeDir::new("public"));

    let [lander_router, app_router, user_router] = [
        lander::create_router(state.clone()),
        app::create_router(state.clone()),
        user::create_router(state.clone()),
    ]
    .map(|router| router.fallback_service(static_service.clone()));

    axum::Router::new()
        .fallback_service(SubdomainRouter::new(lander_router, app_router, user_router))
        .layer(middleware::from_fn_with_state(state, base))
        .layer(LiveReloadLayer::new())
}
