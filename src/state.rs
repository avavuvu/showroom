use aws_sdk_sesv2::Client as SesClient;
use sea_orm::DatabaseConnection;

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

    pub fn user(&self, handle: &str) -> String {
        format!("{}://{}.{}{}", self.scheme(), handle, self.domain, self.port_suffix())
    }

    pub fn cookie(&self) -> String {
        format!(".{}", self.domain)
    }

    pub fn email(&self, handle: &str) -> String {
        format!("{}@{}", handle, self.main_domain)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub jwt_secret: String,
    pub ses: SesClient,
}
