use boutique::Server;
use crate::{routers::*, services::{error_page, subdomain::SubdomainRouter}, state::AppState};

pub fn create_server(state: &AppState) -> (Server, SubdomainRouter) {
    let routes = SubdomainRouter::new(
        lander::create_router(state.clone()),
        app::create_router(state.clone()),
        user::create_router(state.clone()),
        state.urls.domain(),
        state.urls.main_domain(),
    );

    let server = Server::new()
        .file("/favicon.ico", "resources/static/favicon.ico")
        .file("/manifest.webmanifest", "resources/static/manifest.webmanifest")
        .static_dir("/assets", "resources/static/assets")
        .static_dir("/icons", "resources/static/icons")
        .static_dir("/avatars", "resources/static/avatars")
        .error_page_with_state(state.clone(), error_page::render)
        .debug(cfg!(debug_assertions));

    (server, routes)
}
