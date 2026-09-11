use aws_sdk_sesv2::Client as SesClient;
use axum::extract::FromRef;
use boutique::{AuthConfig, AuthState};
use sea_orm::DatabaseConnection;
use crate::config::cloudinary::CloudinaryConfig;

#[derive(Clone)]
pub struct Urls {
    domain: String,      // room.lc — used for subdomain routing
    main_domain: String, // show.room.lc — primary URL and email from-address
    port: String,
    secure: bool,
}

impl Urls {
    pub fn new(domain: impl Into<String>, port: impl Into<String>, main_domain: impl Into<String>) -> Self {
            let secure = !cfg!(debug_assertions);

            Self {
                domain: domain.into(),
                main_domain: main_domain.into(),
                port: port.into(),
                secure,
            }
    }

    fn scheme(&self) -> &str {
        if self.secure { "https" } else { "http" }
    }

    fn port_suffix(&self) -> String {
        if self.secure { String::new() } else { format!(":{}", self.port) }
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn main_domain(&self) -> &str {
        &self.main_domain
    }

    pub fn base(&self) -> String {
        format!("{}://{}{}", self.scheme(), self.main_domain, self.port_suffix())
    }

    pub fn app(&self) -> String {
        format!("{}://app.{}{}", self.scheme(), self.domain, self.port_suffix())
    }

    pub fn publication(&self, slug: &str) -> String {
        format!("{}://{}.{}{}", self.scheme(), slug, self.domain, self.port_suffix())
    }

    pub fn dashboard(&self, slug: &str) -> String {
        format!("{}/{}", self.app(), slug)
    }

    pub fn cookie(&self) -> String {
        format!(".{}", self.domain)
    }

    pub fn email(&self, slug: &str) -> String {
        format!("{}@{}", slug, self.main_domain)
    }

    pub fn auth_config(&self) -> AuthConfig {
        AuthConfig::new(format!("{}/login", self.base()), self.cookie())
            .secure_cookies(self.secure)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub auth: AuthState,
    pub ses: SesClient,
    pub cloudinary: CloudinaryConfig,
}

impl AppState {
    pub fn new(db: DatabaseConnection, ses: SesClient, cloudinary: CloudinaryConfig, urls: Urls, jwt_secret: String) -> Self {
        let auth = AuthState::new(db.clone(), jwt_secret, urls.auth_config());
        Self { db, urls, auth, ses, cloudinary }
    }
}

impl FromRef<AppState> for AuthState {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}
