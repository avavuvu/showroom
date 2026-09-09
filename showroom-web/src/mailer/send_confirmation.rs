type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use aws_sdk_sesv2::{Client, types::{Body, Destination, EmailContent, Message}};

use crate::{mailer::util::convert_ses_content, models::publication::Model as Publication, state::Urls, views::layouts::{confirmation_html, confirmation_text}};

pub async fn send_confirmation(
    client: &Client,
    subscriber_email: &str,
    subscriber_name: Option<&str>,
    token: &str,
    publication: &Publication,
    urls: &Urls,
) -> Result<()> {
    let confirm_url = format!("{}/confirm?token={}", urls.publication(&publication.slug), token);

    let subject = convert_ses_content(format!("Confirm your subscription to {}", publication.name));

    let html = convert_ses_content(confirmation_html(subscriber_name, &confirm_url, &publication.name));
    let text = convert_ses_content(confirmation_text(subscriber_name, &confirm_url, &publication.name));

    let body = Body::builder().html(html).text(text).build();
    let message = Message::builder().subject(subject).body(body).build();
    let email_content = EmailContent::builder().simple(message).build();
    let destination = Destination::builder().to_addresses(subscriber_email).build();

    client
        .send_email()
        .from_email_address(format!("{} <{}>", publication.name, urls.email(&publication.slug)))
        .destination(destination)
        .content(email_content)
        .send()
        .await
        .map_err(|e| format!("Failed to send confirmation email: {e}"))?;

    Ok(())
}
