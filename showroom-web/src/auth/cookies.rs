use axum_extra::extract::cookie::Cookie;
use time::Duration;

pub fn make(key: &str, value: String, duration_hrs: i64, domain: &str) -> Cookie<'static> {
    Cookie::build((key.to_string(), value))
        .domain(domain.to_string())
        .path("/")
        .http_only(true)
        .max_age(Duration::hours(duration_hrs))
        .secure(!cfg!(debug_assertions))
        .build()
}

pub fn remove(key: &str, domain: &str) -> Cookie<'static> {
    Cookie::build((key.to_string(), String::new()))
        .domain(domain.to_string())
        .path("/")
        .build()
}
