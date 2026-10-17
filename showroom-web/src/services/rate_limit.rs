use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError},
    time::{Duration, Instant},
};

use axum::http::HeaderMap;

const PRUNE_AT: usize = 10_000;

#[derive(Clone, Copy)]
pub struct Limit {
    pub count: u32,
    pub window: Duration,
}

impl Limit {
    pub const fn per_hour(count: u32) -> Self {
        Self { count, window: Duration::from_secs(60 * 60) }
    }

    pub const fn per_quarter_hour(count: u32) -> Self {
        Self { count, window: Duration::from_secs(15 * 60) }
    }
}

struct Window {
    started: Instant,
    length: Duration,
    count: u32,
}

#[derive(Clone, Default)]
pub struct RateLimiter {
    windows: Arc<Mutex<HashMap<String, Window>>>,
}

impl RateLimiter {
    pub fn allow(&self, key: impl Into<String>, limit: Limit) -> bool {
        let now = Instant::now();
        let mut windows = self.windows.lock().unwrap_or_else(PoisonError::into_inner);
        if windows.len() >= PRUNE_AT {
            windows.retain(|_, window| now.duration_since(window.started) < window.length);
        }
        let window = windows.entry(key.into()).or_insert(Window { started: now, length: limit.window, count: 0 });
        if now.duration_since(window.started) >= limit.window {
            *window = Window { started: now, length: limit.window, count: 0 };
        }
        window.count = window.count.saturating_add(1);
        window.count <= limit.count
    }

    pub fn allow_all(&self, checks: &[(String, Limit)]) -> bool {
        checks.iter().fold(true, |allowed, (key, limit)| self.allow(key.clone(), *limit) && allowed)
    }
}

pub fn client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.rsplit(',').next())
        .map(str::trim)
        .filter(|ip| !ip.is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stops_after_the_limit_within_a_window() {
        let limiter = RateLimiter::default();
        let limit = Limit::per_hour(2);
        assert!(limiter.allow("a", limit));
        assert!(limiter.allow("a", limit));
        assert!(!limiter.allow("a", limit));
        assert!(limiter.allow("b", limit));
    }
}
