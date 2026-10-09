use aws_sdk_sesv2::Client as SesClient;
use axum::extract::FromRef;
use boutique::{AuthConfig, AuthState};
use sea_orm::DatabaseConnection;
use boutique::cloudinary::Cloudinary;

use crate::{auth, models::user};

#[derive(Clone)]
pub struct Urls {
    domain: String,      // room.lc — used for subdomain routing
    main_domain: String, // show.room.lc — primary URL
    email_domain: String,
    asset_url: Option<String>,
    cloudinary_cloud: String,
    port: String,
    secure: bool,
}

impl Urls {
    pub fn new(domain: impl Into<String>, port: impl Into<String>, main_domain: impl Into<String>) -> Self {
            let secure = !cfg!(debug_assertions);

            let main_domain = main_domain.into();

            Self {
                domain: domain.into(),
                email_domain: main_domain.clone(),
                main_domain,
                asset_url: None,
                cloudinary_cloud: String::new(),
                port: port.into(),
                secure,
            }
    }

    pub fn with_email_domain(mut self, email_domain: impl Into<String>) -> Self {
        self.email_domain = email_domain.into();
        self
    }

    pub fn with_asset_url(mut self, asset_url: impl Into<String>) -> Self {
        self.asset_url = Some(asset_url.into().trim_end_matches('/').to_string());
        self
    }

    pub fn with_cloudinary(mut self, cloud_name: impl Into<String>) -> Self {
        self.cloudinary_cloud = cloud_name.into();
        self
    }

    pub fn asset(&self, path: &str) -> String {
        match &self.asset_url {
            Some(asset_url) => format!("{asset_url}{path}"),
            None => format!("{}{path}", self.base()),
        }
    }

    pub fn cloudinary(&self, public_id: &str, transform: &str) -> String {
        format!("https://res.cloudinary.com/{}/image/upload/{transform}/{public_id}", self.cloudinary_cloud)
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
        format!("{}@{}", slug, self.email_domain)
    }

    pub fn auth_config(&self) -> AuthConfig {
        AuthConfig::default()
            .login_url(format!("{}/login", self.base()))
            .cookie_domain(self.cookie())
            .secure_cookies(self.secure)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub auth: AuthState<user::Model>,
    pub ses: SesClient,
    pub cloudinary: Cloudinary,
}

impl AppState {
    pub fn new(db: DatabaseConnection, ses: SesClient, cloudinary: Cloudinary, urls: Urls, secret: String) -> Self {
        let auth = AuthState::with_config(auth::Store::new(db.clone()), secret, urls.auth_config());
        Self { db, urls, auth, ses, cloudinary }
    }
}

impl FromRef<AppState> for AuthState<user::Model> {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}
