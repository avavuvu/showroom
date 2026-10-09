mod dashboard_shell;
mod email;
mod page;
mod shell;

pub use boutique::views::{Metadata, Head, base};
pub use dashboard_shell::dashboard_shell;
pub use email::{base_email_layout, confirmation_html, NewsletterBody, NewsletterTemplateData, confirmation_text, generate_subscriber_data};
pub use page::page;
pub use shell::shell;
