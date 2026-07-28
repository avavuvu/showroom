type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use aws_sdk_sesv2::{
    Client,
    types::{
        BulkEmailContent, BulkEmailEntry, Destination,
        EmailTemplateContent, MessageHeader, ReplacementEmailContent,
        ReplacementTemplate, Template,
    },
};

use crate::{
    models::{newsletter::Model as Newsletter, subscriber::Model as Subscriber}, renderer::{email::{ThemeVariables, render_email}, plain_text}, state::Urls, views::layouts::{NewsletterTemplateData, generate_subscriber_data, newsletter_template},
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

    let rendered_content = render_email(&newsletter.content, ThemeVariables::default());
    let date = newsletter.created_at.format("%B %-d, %Y").to_string();
    let read_online_url = format!("{}/{}", urls.user(author_handle), newsletter.slug);
    let html = newsletter_template(
        &newsletter.title,
        newsletter.subtitle.as_deref(),
        &author_handle,
        &date,
        &read_online_url,
        &urls.user(author_handle),
        None,
        rendered_content,
    );
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
        .map_err(|e| format!("Failed to create SES template: {e}"))?;

    let default_data = serde_json::to_string(
        &NewsletterTemplateData::default()
    ).unwrap();

    let bulk_content = BulkEmailContent::builder()
        .template(
            Template::builder()
                .template_name(&template_name)
                .template_data(default_data)
                .build(),
        )
        .build();

    let from = urls.email(author_handle);
    println!("{from}");

    let entries: Vec<BulkEmailEntry> = subscribers
        .iter()
        .map(|sub| {
            let unsubscribe_url = format!(
                "{}/unsubscribe?token={}",
                urls.user(author_handle),
                sub.token
            );

            let data = serde_json::to_string(
                &generate_subscriber_data(
                    sub.name.as_deref(),
                    &unsubscribe_url)
            ).unwrap();

            let destination = Destination::builder().to_addresses(&sub.email).build();

            let list_unsubscribe = MessageHeader::builder()
                .name("List-Unsubscribe")
                .value(format!("<{}>", unsubscribe_url))
                .build()
                .unwrap();

            let list_unsubscribe_post = MessageHeader::builder()
                .name("List-Unsubscribe-Post")
                .value("List-Unsubscribe=One-Click")
                .build()
                .unwrap();

            let replacement = ReplacementTemplate::builder()
                .replacement_template_data(data)
                .build();

            let replacement_content = ReplacementEmailContent::builder()
                .replacement_template(replacement)
                .build();

            BulkEmailEntry::builder()
                .destination(destination)
                .replacement_email_content(replacement_content)
                .replacement_headers(list_unsubscribe)
                .replacement_headers(list_unsubscribe_post)
                .build()
        })
        .collect();

    let mut failures: Vec<String> = Vec::new();

    for chunk in entries.chunks(50) {
        let result = client
            .send_bulk_email()
            .from_email_address(&from)
            .default_content(bulk_content.clone())
            .set_bulk_email_entries(Some(chunk.to_vec()))
            .send()
            .await
            .map_err(|e| format!("Failed to send bulk email chunk: {e}"))?;

        println!("{:#?}", result);

        for r in result.bulk_email_entry_results() {
            let status = r.status().map(|s| s.as_str()).unwrap_or("UNKNOWN");
            if status != "SUCCESS" {
                let error = r.error().unwrap_or("no error message");
                failures.push(format!("{status}: {error}"));
            }
        }
    }

    if failures.is_empty() {
        client
            .delete_email_template()
            .template_name(&template_name)
            .send()
            .await
            .map_err(|e| format!("Failed to delete SES template: {e}"))?;
    } else {
        return Err(format!("One or more recipients failed:\n{}", failures.join("\n")).into());
    }

    Ok(())
}
