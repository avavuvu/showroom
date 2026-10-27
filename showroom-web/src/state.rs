use std::sync::Arc;

use aws_sdk_sesv2::Client as SesClient;
use boutique::{AuthConfig, AuthState, HasAuth, store::sea_orm::SeaOrmStore};
use sea_orm::DatabaseConnection;
use boutique::cloudinary::Cloudinary;
use tokio::sync::Notify;

use crate::{images::{ImageSize, cloudinary_url}, models::user, services::{music::MusicClient, rate_limit::RateLimiter, youtube::YoutubeClient}};

#[derive(Clone, Debug)]
pub struct Urls {
    pub site_domain: String,
    pub publication_domain: String,
    pub old_site_domains: Vec<String>,
    pub publication_email_domain: Option<String>,
    pub asset_url: Option<String>,
    pub cloudinary_cloud: String,
    pub https: bool,
    pub public_port: Option<String>,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            site_domain: "localtest.me".to_string(),
            publication_domain: "localtest.me".to_string(),
            old_site_domains: Vec::new(),
            publication_email_domain: None,
            asset_url: None,
            cloudinary_cloud: String::new(),
            https: false,
            public_port: Some("3000".to_string()),
        }
    }
}

impl Urls {
    pub fn from_env() -> Self {
        let var = |name: &str, old_name: Option<&str>| -> Option<String> {
            let value = std::env::var(name).ok().or_else(|| {
                let old_name = old_name?;
                let value = std::env::var(old_name).ok()?;
                tracing::warn!("{old_name} is deprecated, rename it to {name}");
                Some(value)
            });
            value.map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
        };

        let site_domain = var("SITE_DOMAIN", Some("MAIN_DOMAIN")).unwrap_or_else(|| "localtest.me".to_string());
        let https = var("HTTPS", None).map_or(!cfg!(debug_assertions), |value| value == "true");
        let public_port = var("PUBLIC_PORT", None).or_else(|| (!https).then(|| var("PORT", None).unwrap_or_else(|| "3000".to_string())));

        Self {
            publication_domain: var("PUBLICATION_DOMAIN", Some("DOMAIN")).unwrap_or_else(|| site_domain.clone()),
            site_domain,
            old_site_domains: var("OLD_SITE_DOMAINS", Some("LEGACY_SITE_DOMAINS"))
                .unwrap_or_default()
                .split(',')
                .map(|domain| domain.trim().to_string())
                .filter(|domain| !domain.is_empty())
                .collect(),
            publication_email_domain: var("PUBLICATION_EMAIL_DOMAIN", Some("EMAIL_DOMAIN")),
            asset_url: var("ASSET_URL", None).map(|url| url.trim_end_matches('/').to_string()),
            cloudinary_cloud: String::new(),
            https,
            public_port,
        }
    }

    fn origin(&self, host: &str) -> String {
        let scheme = if self.https { "https" } else { "http" };
        match &self.public_port {
            Some(port) => format!("{scheme}://{host}:{port}"),
            None => format!("{scheme}://{host}"),
        }
    }

    pub fn app_domain(&self) -> String {
        format!("app.{}", self.site_domain)
    }

    pub fn base(&self) -> String {
        self.origin(&self.site_domain)
    }

    pub fn app(&self) -> String {
        self.origin(&self.app_domain())
    }

    pub fn publication(&self, slug: &str) -> String {
        self.origin(&format!("{slug}.{}", self.publication_domain))
    }

    pub fn dashboard(&self, slug: &str) -> String {
        format!("{}/{}", self.app(), slug)
    }

    pub fn asset(&self, path: &str) -> String {
        match &self.asset_url {
            Some(asset_url) => format!("{asset_url}{path}"),
            None => format!("{}{path}", self.base()),
        }
    }

    pub fn image(&self, public_id: &str, size: ImageSize) -> String {
        cloudinary_url(&self.cloudinary_cloud, public_id, size)
    }

    pub fn publication_email(&self, slug: &str) -> String {
        format!("{slug}@{}", self.publication_email_domain.as_deref().unwrap_or(&self.publication_domain))
    }

    pub fn account_email(&self, name: &str) -> String {
        format!("{name}@{}", self.site_domain)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub auth: AuthState<user::Store>,
    pub ses: SesClient,
    pub cloudinary: Cloudinary,
    pub sends: Arc<Notify>,
    pub music: MusicClient,
    pub youtube: YoutubeClient,
    pub limits: RateLimiter,
}

impl AppState {
    pub fn new(db: DatabaseConnection, ses: SesClient, cloudinary: Cloudinary, urls: Urls, secret: String) -> Self {
        let auth_config = AuthConfig::default()
            .login_url(format!("{}/login", urls.base()))
            .cookie_domain(format!(".{}", urls.site_domain))
            .secure_cookies(urls.https);
        let auth = AuthState::new(SeaOrmStore::new(db.clone()), secret).with_config(auth_config);
        Self { db, urls, auth, ses, cloudinary, sends: Arc::new(Notify::new()), music: MusicClient::new(), youtube: YoutubeClient::new(), limits: RateLimiter::default() }
    }
}

impl HasAuth for AppState {
    type Store = user::Store;

    fn auth(&self) -> &AuthState<user::Store> {
        &self.auth
    }
}
