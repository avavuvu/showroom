use axum::middleware;
use sea_orm::DatabaseConnection;
use axum::http::{HeaderValue, header};
use tower::ServiceBuilder;
use tower_http::{services::{ServeDir, ServeFile}, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;
use crate::{auth::middleware::base, routers::*, state::{AppState, Urls}, services::subdomain::SubdomainRouter};

pub fn create_service(db: DatabaseConnection, ses: aws_sdk_sesv2::Client, domain: &str, port: &str, main_domain: &str, jwt_secret: String) -> axum::Router {
    let state = AppState { db, ses, urls: Urls::new(domain, port, main_domain), jwt_secret };

    let no_cache = |dir| ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .service(ServeDir::new(dir));

    let lander_router = lander::create_router(state.clone());
    let app_router = app::create_router(state.clone());
    let user_router = user::create_router(state.clone());

    let router = axum::Router::new()
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .nest_service("/css", no_cache("resources/css"))
        .nest_service("/assets", no_cache("public/assets"))
        .nest_service("/icons", no_cache("public/icons"))
        .fallback_service(SubdomainRouter::new(lander_router, app_router, user_router, domain, main_domain))
        .layer(middleware::from_fn_with_state(state, base));

    #[cfg(debug_assertions)]
    let router = router.layer(LiveReloadLayer::new());

    router
}
