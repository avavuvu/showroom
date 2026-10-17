use aws_sdk_sesv2::{
    Client,
    error::{ProvideErrorMetadata, SdkError},
    types::{
        BulkEmailContent, BulkEmailEntry, BulkEmailStatus, Destination,
        EmailTemplateContent, MessageHeader, ReplacementEmailContent,
        ReplacementTemplate, Template,
    },
};

use crate::{
    document::Document,
    mailer::util::{from_address, is_retryable, ses_error},
    models::{newsletter::Model as Newsletter, publication::Model as Publication, subscriber::Model as Subscriber},
    renderer::{email::render_email, personalize, plain_text},
    state::Urls,
    views::layouts::{NewsletterBody, NewsletterTemplateData, base_email_layout, generate_subscriber_data},
};
use boutique::html;
use chrono::{DateTime, FixedOffset};

pub const BATCH_SIZE: usize = 50;

#[derive(Debug)]
pub struct MailError {
    pub message: String,
    pub retryable: bool,
}

impl MailError {
    fn permanent(message: impl Into<String>) -> Self {
        Self { message: message.into(), retryable: false }
    }

    fn ses<E, R>(action: &str, error: &SdkError<E, R>) -> Self
    where
        E: ProvideErrorMetadata + std::error::Error + 'static,
        R: std::fmt::Debug,
    {
        Self { message: ses_error(action, error), retryable: is_retryable(error) }
    }
}

impl std::fmt::Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MailError {}

pub enum Outcome {
    Sent(Option<String>),
    Retry(String),
    Failed(String),
}

pub struct Delivery<'a> {
    pub subscriber: &'a Subscriber,
    pub outcome: Outcome,
}

pub struct NewsletterMailing<'a> {
    client: &'a Client,
    template_name: String,
    from: String,
    publication_url: String,
}

pub struct RenderedEmail {
    pub subject: String,
    pub html: String,
    pub text: String,
}

pub fn render(newsletter: &Newsletter, publication: &Publication, date: DateTime<FixedOffset>, urls: &Urls) -> Result<RenderedEmail, MailError> {
    let publication_url = urls.publication(&publication.slug);
    let theme = publication.theme().email();
    let document = Document::from_value(&newsletter.content)
        .map_err(|e| MailError::permanent(format!("Newsletter content is not valid: {e}")))?;
    let rendered_content: Vec<_> = render_email(&document, theme, urls).into_iter().map(|block| block.map(personalize::template)).collect();
    let date = date.format("%B %-d, %Y").to_string();
    let read_online_url = format!("{}/{}", publication_url, newsletter.slug);
    let title = personalize::escape(newsletter.display_title());
    let subtitle = newsletter.subtitle.as_deref().map(personalize::escape);
    let publication_name = personalize::escape(&publication.name);
    let html = base_email_layout(
        &title,
        subtitle.as_deref(),
        theme,
        html! {
            NewsletterBody(
                title = &title,
                maybe_subtitle = subtitle.as_deref(),
                publication_name = &publication_name,
                date = &date,
                read_online_url = &read_online_url,
                publication_url = &publication_url,
                picture = &publication.email_picture_url(urls),
                layout = theme.layout,
                content = &rendered_content,
            );
        },
    );
    let text = format!(
        "{}\n\n---\nRead online: {}\nUnsubscribe: {{{{{{unsubscribe_url}}}}}}",
        plain_text::render(&document),
        personalize::escape(&read_online_url),
    );

    Ok(RenderedEmail { subject: title.into_owned(), html: html.into_string(), text })
}

impl<'a> NewsletterMailing<'a> {
    pub async fn prepare(
        client: &'a Client,
        newsletter_id: &str,
        email: &RenderedEmail,
        publication: &Publication,
        urls: &Urls,
    ) -> Result<Self, MailError> {
        let template_name = format!("newsletter-{}-{}", newsletter_id, uuid::Uuid::new_v4().simple());

        client
            .create_email_template()
            .template_name(&template_name)
            .template_content(
                EmailTemplateContent::builder()
                    .subject(&email.subject)
                    .html(&email.html)
                    .text(&email.text)
                    .build(),
            )
            .send()
            .await
            .map_err(|e| MailError::ses("Failed to create the email template", &e))?;

        Ok(Self {
            client,
            template_name,
            from: from_address(&publication.name, &urls.email(&publication.slug)),
            publication_url: urls.publication(&publication.slug),
        })
    }

    pub async fn send<'s>(&self, subscribers: &'s [Subscriber]) -> Result<Vec<Delivery<'s>>, MailError> {
        if subscribers.len() > BATCH_SIZE {
            return Err(MailError::permanent(format!("A batch can have at most {BATCH_SIZE} recipients")));
        }
        if subscribers.is_empty() {
            return Ok(Vec::new());
        }

        let default_data = serde_json::to_string(&NewsletterTemplateData::default())
            .map_err(|e| MailError::permanent(e.to_string()))?;
        let default_content = BulkEmailContent::builder()
            .template(
                Template::builder()
                    .template_name(&self.template_name)
                    .template_data(default_data)
                    .build(),
            )
            .build();

        let entries = subscribers
            .iter()
            .map(|subscriber| self.entry(subscriber))
            .collect::<Result<Vec<_>, _>>()?;

        let response = self
            .client
            .send_bulk_email()
            .from_email_address(&self.from)
            .default_content(default_content)
            .set_bulk_email_entries(Some(entries))
            .send()
            .await
            .map_err(|e| MailError::ses("Failed to send the newsletter", &e))?;

        let results = response.bulk_email_entry_results();
        if results.len() != subscribers.len() {
            return Err(MailError::permanent(format!(
                "Amazon SES returned {} results for {} recipients",
                results.len(),
                subscribers.len()
            )));
        }

        Ok(subscribers
            .iter()
            .zip(results)
            .map(|(subscriber, result)| {
                let error = || {
                    result
                        .error()
                        .map(str::to_string)
                        .unwrap_or_else(|| result.status().map(|s| s.as_str()).unwrap_or("Unknown error").to_string())
                };
                let outcome = match result.status() {
                    Some(BulkEmailStatus::Success) => Outcome::Sent(result.message_id().map(str::to_string)),
                    Some(BulkEmailStatus::AccountThrottled | BulkEmailStatus::TransientFailure) => Outcome::Retry(error()),
                    _ => Outcome::Failed(error()),
                };
                Delivery { subscriber, outcome }
            })
            .collect())
    }

    pub async fn finish(self) -> Result<(), MailError> {
        self.client
            .delete_email_template()
            .template_name(&self.template_name)
            .send()
            .await
            .map_err(|e| MailError::ses("Failed to delete the email template", &e))?;
        Ok(())
    }

    fn entry(&self, subscriber: &Subscriber) -> Result<BulkEmailEntry, MailError> {
        let unsubscribe_url = format!("{}/unsubscribe?token={}", self.publication_url, subscriber.token);
        let data = serde_json::to_string(&generate_subscriber_data(subscriber.name.as_deref(), &unsubscribe_url))
            .map_err(|e| MailError::permanent(e.to_string()))?;

        let list_unsubscribe = MessageHeader::builder()
            .name("List-Unsubscribe")
            .value(format!("<{unsubscribe_url}>"))
            .build()
            .map_err(|e| MailError::permanent(e.to_string()))?;

        let list_unsubscribe_post = MessageHeader::builder()
            .name("List-Unsubscribe-Post")
            .value("List-Unsubscribe=One-Click")
            .build()
            .map_err(|e| MailError::permanent(e.to_string()))?;

        Ok(BulkEmailEntry::builder()
            .destination(Destination::builder().to_addresses(&subscriber.email).build())
            .replacement_email_content(
                ReplacementEmailContent::builder()
                    .replacement_template(ReplacementTemplate::builder().replacement_template_data(data).build())
                    .build(),
            )
            .replacement_headers(list_unsubscribe)
            .replacement_headers(list_unsubscribe_post)
            .build())
    }
}
