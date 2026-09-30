use boutique::{Server, assets::manifest};
use crate::{models, routers::*, services::subdomain::SubdomainRouter, state::AppState};

pub const BUILD_ROUTE: &str = "/build";
pub const BUILD_DIR: &str = "public/build";

pub fn create_server(state: &AppState) -> (Server<models::user::Model>, SubdomainRouter) {
    manifest::init(BUILD_ROUTE, BUILD_DIR);

    let lander_router = lander::create_router(state.clone());
    let app_router = app::create_router(state.clone());
    let user_router = user::create_router(state.clone());

    let routes = SubdomainRouter::new(
        lander_router,
        app_router,
        user_router,
        state.urls.domain(),
        state.urls.main_domain(),
    );

    let server = Server::new(state.auth.clone())
        .file("/favicon.ico", "resources/static/favicon.ico")
        .static_dir("/assets", "resources/static/assets")
        .static_dir("/icons", "resources/static/icons")
        .static_dir(BUILD_ROUTE, BUILD_DIR)
        .debug(cfg!(debug_assertions));

    (server, routes)
}
