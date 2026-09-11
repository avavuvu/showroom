#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub login_url: String,
    pub cookie_domain: String,
    pub secure_cookies: bool,
    pub jwt_ttl_hours: i64,
    pub refresh_ttl_hours: i64,
    pub reset_ttl_hours: i64,
}

impl AuthConfig {
    pub fn new(login_url: impl Into<String>, cookie_domain: impl Into<String>) -> Self {
        Self {
            login_url: login_url.into(),
            cookie_domain: cookie_domain.into(),
            secure_cookies: true,
            jwt_ttl_hours: 1,
            refresh_ttl_hours: 30 * 24,
            reset_ttl_hours: 1,
        }
    }

    pub fn secure_cookies(mut self, secure: bool) -> Self {
        self.secure_cookies = secure;
        self
    }

    pub fn jwt_ttl_hours(mut self, hours: i64) -> Self {
        self.jwt_ttl_hours = hours;
        self
    }

    pub fn refresh_ttl_hours(mut self, hours: i64) -> Self {
        self.refresh_ttl_hours = hours;
        self
    }

    pub fn reset_ttl_hours(mut self, hours: i64) -> Self {
        self.reset_ttl_hours = hours;
        self
    }
}
