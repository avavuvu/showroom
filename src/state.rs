use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Urls {
    /// Root domain url
    pub base: String,
    /// app. subdomain
    pub app: String,
    /// Cookie domain shared across all subdomains
    pub cookie: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub jwt_secret: String,
}
