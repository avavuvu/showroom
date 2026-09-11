use boutique::Server;
use crate::{routers::*, services::subdomain::SubdomainRouter, state::AppState};

pub fn create_server(state: &AppState) -> (Server, SubdomainRouter) {
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
        .file("/favicon.ico", "public/favicon.ico")
        .static_dir("/css", "resources/css")
        .static_dir("/assets", "public/assets")
        .static_dir("/icons", "public/icons")
        .debug(cfg!(debug_assertions));

    (server, routes)
}
