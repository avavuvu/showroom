use sea_orm::DatabaseConnection;
use axum::http::{HeaderValue, header};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use crate::{routers::*, state::{AppState, Urls}, services::subdomain::SubdomainRouter};

pub fn create_service(db: DatabaseConnection, domain: &str, port: &str, is_production: bool) -> axum::Router {
    let urls = Urls {
        base: format!("http://{domain}:{port}"),
        app: format!("http://app.{domain}:{port}"),
    };
    let state = AppState { db, urls };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_domain(format!(".{domain}"))
        .with_secure(is_production);

    let static_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .service(ServeDir::new("public"));

    let [lander_router, app_router, user_router] = [
        lander::create_router(state.clone(), session_layer.clone()),
        app::create_router(state.clone(), session_layer.clone()),
        user::create_router(state, session_layer),
    ].map(|router| router.fallback_service(static_service.clone()));

    axum::Router::new()
        .fallback_service(SubdomainRouter::new(lander_router, app_router, user_router))
        .layer(LiveReloadLayer::new())
}
