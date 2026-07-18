type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use aws_sdk_sesv2::{Client, types::{Body, Destination, EmailContent, Message}};
use crate::{mailer::util::convert_ses_content, state::Urls};

pub async fn send_password_reset(
    client: &Client,
    to_email: &str,
    reset_url: &str,
    urls: &Urls,
) -> Result<()> {
    let subject = convert_ses_content("Reset your Showroom password".to_string());

    let html = convert_ses_content(format!(
        "<p>Click the link below to set a new password. This link expires in 1 hour.</p>\
         <p><a href=\"{reset_url}\">{reset_url}</a></p>"
    ));
    let text = convert_ses_content(format!(
        "Click the link below to set a new password. This link expires in 1 hour.\n\n{reset_url}"
    ));

    let body = Body::builder().html(html).text(text).build();
    let message = Message::builder().subject(subject).body(body).build();
    let email_content = EmailContent::builder().simple(message).build();
    let destination = Destination::builder().to_addresses(to_email).build();

    client
        .send_email()
        .from_email_address(urls.email("noreply"))
        .destination(destination)
        .content(email_content)
        .send()
        .await
        .map_err(|e| format!("Failed to send password reset email: {e:#?}"))?;

    Ok(())
}
