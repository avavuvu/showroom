mod dashboard_shell;
mod email;
mod page;
mod shell;

pub use boutique::views::{Metadata, Head, base};
pub use dashboard_shell::dashboard_shell;
pub use email::{confirmation_html, newsletter_template, NewsletterTemplateData, confirmation_text, generate_subscriber_data};
pub use page::page;
pub use shell::shell;
