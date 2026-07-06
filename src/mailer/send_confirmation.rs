type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use aws_sdk_sesv2::{Client, types::{Body, Destination, EmailContent, Message}};

use crate::{mailer::util::convert_ses_content, state::Urls, views::layouts::{confirmation_html, confirmation_text}};

pub async fn send_confirmation(
    client: &Client,
    subscriber_email: &str,
    subscriber_name: Option<&str>,
    token: &str,
    author_handle: &str,
    urls: &Urls,
) -> Result<()> {
    let confirm_url = format!("{}/confirm?token={}", urls.user(author_handle), token);

    let subject = convert_ses_content(format!("Confirm your subscription to @{author_handle}"));

    let html = convert_ses_content(confirmation_html(subscriber_name, &confirm_url, author_handle));
    let text = convert_ses_content(confirmation_text(subscriber_name, &confirm_url, author_handle));

    let body = Body::builder().html(html).text(text).build();
    let message = Message::builder().subject(subject).body(body).build();
    let email_content = EmailContent::builder().simple(message).build();
    let destination = Destination::builder().to_addresses(subscriber_email).build();

    client
        .send_email()
        .from_email_address(urls.email(author_handle))
        .destination(destination)
        .content(email_content)
        .send()
        .await
        .map_err(|e| format!("Failed to send confirmation email: {e}"))?;

    Ok(())
}
