pub mod api;
pub mod feeds;
pub mod sitemap;
pub mod auth;
pub mod comments;
pub mod dashboard;
pub mod error404;
pub mod home;
pub mod subscriber;
pub mod user;
mod passthrough;
pub use passthrough::passthrough;

pub fn is_safe_location(location: &str) -> bool {
    location.bytes().all(|byte| byte.is_ascii_graphic() && byte != b'\\')
}

pub fn is_local_path(path: &str) -> bool {
    path.starts_with('/') && !path.starts_with("//") && is_safe_location(path)
}
