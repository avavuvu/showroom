mod base;
mod email;
mod shell;

pub use base::base;
pub use shell::shell;
pub use email::{base_email_layout, confirmation_html, newsletter_template, NewsletterTemplateData, confirmation_text, generate_subscriber_data};
