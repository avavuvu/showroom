mod base;
mod dashboard_shell;
mod email;
mod shell;
pub mod view_context;

pub use view_context::{ViewContext, Metadata, OgType};
pub use base::base;
pub use dashboard_shell::dashboard_shell;
pub use shell::shell;
pub use email::{base_email_layout, confirmation_html, newsletter_template, NewsletterTemplateData, confirmation_text, generate_subscriber_data};
