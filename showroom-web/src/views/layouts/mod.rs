mod dashboard_shell;
mod email;
mod shell;

pub use boutique::views::{Metadata, ViewContext, base};
pub use dashboard_shell::dashboard_shell;
pub use shell::shell;
pub use email::{confirmation_html, newsletter_template, NewsletterTemplateData, confirmation_text, generate_subscriber_data};

pub fn page(title: impl Into<String>) -> ViewContext {
    ViewContext::new(title)
        .favicon("/favicon.ico")
        .stylesheet("/css/app.css")
        .stylesheet("/css/prose.css")
}

pub const ALPINE_ENTRY: &str = "/assets/alpine.js";
pub const ISLANDS_ENTRY: &str = "/assets/islands.js";
