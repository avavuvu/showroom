type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use aws_sdk_sesv2::{Client, types::{Body, Destination, EmailContent, Message}};

use crate::{
    mailer::util::{convert_ses_content, from_address, ses_error},
    models::publication::Model as Publication,
    state::Urls,
    views::comments::email::{CommentConfirmation, comment_confirmation_html, comment_confirmation_text},
};

pub async fn send_comment_confirmation(client: &Client, email: &str, confirmation: &CommentConfirmation<'_>, publication: &Publication, urls: &Urls) -> Result<()> {
    let subject = if confirmation.subscribes {
        format!("Confirm your comment and subscription to {}", publication.name)
    } else {
        format!("Publish your comment on {}", confirmation.newsletter_title)
    };
    let html = convert_ses_content(comment_confirmation_html(confirmation, publication.theme().email()));
    let text = convert_ses_content(comment_confirmation_text(confirmation));

    let body = Body::builder().html(html).text(text).build();
    let message = Message::builder().subject(convert_ses_content(subject)).body(body).build();
    let email_content = EmailContent::builder().simple(message).build();
    let destination = Destination::builder().to_addresses(email).build();

    client
        .send_email()
        .from_email_address(from_address(&publication.name, &urls.publication_email(&publication.slug)))
        .destination(destination)
        .content(email_content)
        .send()
        .await
        .map_err(|e| ses_error("Failed to send the comment confirmation email", &e))?;

    Ok(())
}
