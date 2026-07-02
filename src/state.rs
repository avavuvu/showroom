use aws_sdk_sesv2::Client as SesClient;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Urls {
    domain: String,
    email_domain: String,
    port: String,
    secure: bool,

}

impl Urls {
    pub fn new(domain: impl Into<String>, port: impl Into<String>, email_domain: impl Into<String>) -> Self {
        let secure = !cfg!(debug_assertions);

        Self {
            domain: domain.into(),
            port: port.into(),
            email_domain: email_domain.into(),
            secure,
        }
    }

    fn scheme(&self) -> &str {
        if self.secure { "https" } else { "http" }
    }

    pub fn base(&self) -> String {
        format!("{}://{}:{}", self.scheme(), self.domain, self.port)
    }

    pub fn app(&self) -> String {
        format!("{}://app.{}:{}", self.scheme(), self.domain, self.port)
    }

    pub fn user(&self, handle: &str) -> String {
        format!("{}://{}.{}:{}", self.scheme(), handle, self.domain, self.port)
    }

    pub fn cookie(&self) -> String {
        format!(".{}", self.domain)
    }

    pub fn email(&self, handle: &str) -> String {
        format!("{}@{}", handle, self.email_domain)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub urls: Urls,
    pub jwt_secret: String,
    pub ses: SesClient,
}
