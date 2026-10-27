use std::time::Duration;

use boutique::{Cache, Server};
use crate::{routers::*, services::{error_page, subdomain::SubdomainRouter}, state::AppState};

const DAY: u64 = 24 * 60 * 60;

pub fn create_server(state: &AppState) -> (Server, SubdomainRouter) {
    let routes = SubdomainRouter::new(
        lander::create_router(state.clone()),
        app::create_router(state.clone()),
        user::create_router(state.clone()),
        state.urls.clone(),
    );

    let server = Server::new()
        .file("/favicon.ico", "resources/static/favicon.ico")
        .file("/manifest.webmanifest", "resources/static/manifest.webmanifest")
        .static_dir("/assets", "resources/static/assets")
        .static_dir_with_cache("/icons", "resources/static/icons", Cache::MaxAge(Duration::from_secs(7 * DAY)))
        .static_dir_with_cache("/avatars", "resources/static/avatars", Cache::MaxAge(Duration::from_secs(30 * DAY)))
        .error_page_with_state(state.clone(), error_page::render)
        .debug(cfg!(debug_assertions));

    (server, routes)
}
