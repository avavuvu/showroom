use anyhow::{Context, Result, bail};
use aws_sdk_sesv2::{
    Client,
    types::{
        Body, BulkEmailContent, BulkEmailEntry, Content, Destination,
        EmailContent, EmailTemplateContent, Message, ReplacementEmailContent,
        ReplacementTemplate, Template,
    },
};
use maud::PreEscaped;
use serde_json::json;

use crate::{
    models::{newsletter::Model as Newsletter, subscriber::Model as Subscriber}, renderer::{html::{ThemeVariables, render_email}, plain_text}, state::Urls, views::{self, layouts::{confirmation_html, confirmation_text, email_layout}},
};

pub async fn send_newsletter(
    client: &Client,
    newsletter: &Newsletter,
    author_handle: &str,
    subscribers: &[Subscriber],
    urls: &Urls,
) -> Result<()> {
    if subscribers.is_empty() {
        return Ok(());
    }

    let template_name = format!("newsletter-{}", newsletter.id);

    let rendered_html = render_email(&newsletter.content, ThemeVariables::default());
    let html = email_layout(&newsletter.title, None, PreEscaped(rendered_html));
    let text = &plain_text::render(&newsletter.content);

    client
        .create_email_template()
        .template_name(&template_name)
        .template_content(
            EmailTemplateContent::builder()
                .subject(&newsletter.title)
                .html(html)
                .text(text)
                .build(),
        )
        .send()
        .await
        .context("Failed to create SES template")?;

    let default_data = json!({ "greeting": "", "unsubscribe": "" }).to_string();

    let bulk_content = BulkEmailContent::builder()
        .template(
            Template::builder()
                .template_name(&template_name)
                .template_data(default_data)
                .build(),
        )
        .build();

    let from = urls.from_email(author_handle);

    let entries: Vec<BulkEmailEntry> = subscribers
        .iter()
        .map(|sub| {
            let unsubscribe_url = format!(
                "{}/unsubscribe?token={}",
                urls.user(author_handle),
                sub.token
            );

            let data = json!({
                "greeting": sub.name.as_deref().map(|n| format!("Hi {n},")).unwrap_or_default(),
                "unsubscribe": format!("<a href=\"{unsubscribe_url}\" style=\"color:#888888;\">Unsubscribe</a>"),
            })
            .to_string();

            BulkEmailEntry::builder()
                .destination(Destination::builder().to_addresses(&sub.email).build())
                .replacement_email_content(
                    ReplacementEmailContent::builder()
                        .replacement_template(
                            ReplacementTemplate::builder()
                                .replacement_template_data(data)
                                .build(),
                        )
                        .build(),
                )
                .build()
        })
        .collect();

    let mut all_succeeded = true;

    for chunk in entries.chunks(50) {
        let result = client
            .send_bulk_email()
            .from_email_address(&from)
            .default_content(bulk_content.clone())
            .set_bulk_email_entries(Some(chunk.to_vec()))
            .send()
            .await
            .context("Failed to send bulk email chunk")?;

        if result
            .bulk_email_entry_results()
            .iter()
            .any(|r| r.status().map(|s| s.as_str()) != Some("SUCCESS"))
        {
            all_succeeded = false;
        }
    }

    if all_succeeded {
        client
            .delete_email_template()
            .template_name(&template_name)
            .send()
            .await
            .context("Failed to delete SES template")?;
    } else {
        bail!("One or more recipients failed");
    }

    Ok(())
}

fn convert_ses_content(data: impl Into<String>) -> Content {
    Content::builder().data(data.into()).build().unwrap()
}

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
        .from_email_address(urls.from_email(author_handle))
        .destination(destination)
        .content(email_content)
        .send()
        .await
        .context("Failed to send confirmation email")?;

    Ok(())
}
