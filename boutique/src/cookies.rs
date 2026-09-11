use axum_extra::extract::cookie::Cookie;
use time::Duration;

use crate::config::AuthConfig;

pub const JWT: &str = "jwt";
pub const REFRESH: &str = "refresh";

pub fn make(key: &str, value: String, duration_hrs: i64, config: &AuthConfig) -> Cookie<'static> {
    Cookie::build((key.to_string(), value))
        .domain(config.cookie_domain.clone())
        .path("/")
        .http_only(true)
        .max_age(Duration::hours(duration_hrs))
        .secure(config.secure_cookies)
        .build()
}

pub fn remove(key: &str, config: &AuthConfig) -> Cookie<'static> {
    Cookie::build((key.to_string(), String::new()))
        .domain(config.cookie_domain.clone())
        .path("/")
        .build()
}
