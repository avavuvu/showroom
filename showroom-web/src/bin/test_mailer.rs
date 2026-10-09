use chrono::Utc;
use dotenvy::dotenv;
use serde_json::json;
use showroom_web::{
    mailer,
    models::{newsletter::Model as Newsletter, publication::Model as Publication, subscriber::Model as Subscriber},
    state::Urls,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let domain = "showroom.you".to_string();
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let to_email = "avadinhvu@gmail.com".to_string();

    let aws_config = aws_config::load_from_env().await;
    let client = aws_sdk_sesv2::Client::new(&aws_config);
    let urls = Urls::new(domain, port, "");

    let publication = Publication {
        id: "test-publication".to_string(),
        owner_id: "test-user".to_string(),
        slug: "test".to_string(),
        name: "test's room".to_string(),
        description: None,
        theme: None,
        greeting: showroom_web::renderer::greeting::DEFAULT.to_string(),
        image: showroom_web::models::publication::default_picture(0),
        banner: None,
        pictures: Default::default(),
        is_default: true,
        created_at: Utc::now().fixed_offset(),
        updated_at: Utc::now().fixed_offset(),
    };

    let newsletter = Newsletter {
        id: "test-05".to_string(),
        publication_id: publication.id.clone(),
        title: "My Test Newsletter".to_string(),
        slug: "test".to_string(),
        subtitle: Some("Testing the mailer".to_string()),
        content: json!({
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [{ "type": "text", "text": "This is a test email. If you received this, the mailer is working." }]
                }
            ]
        }),
        revision: 0,
        published_at: None,
        send_started_at: None,
        created_at: Utc::now().fixed_offset(),
        updated_at: Utc::now().fixed_offset(),
    };

    let subscribers = vec![Subscriber {
        token: "test-token-02".to_string(),
        publication_id: publication.id.clone(),
        name: Some("Ava".to_string()),
        email: to_email.clone(),
        is_confirmed: true,
        created_at: Utc::now().fixed_offset(),
    }];

    println!("Sending test newsletter to {to_email}...");
    println!("Creating SES template...");

    let mailing = mailer::NewsletterMailing::prepare(&client, &newsletter, &publication, &urls).await?;
    let deliveries = mailing.send(&subscribers).await;
    mailing.finish().await?;

    for delivery in deliveries? {
        match delivery.outcome {
            mailer::Outcome::Sent(message_id) => println!("Sent to {} ({})", delivery.subscriber.email, message_id.unwrap_or_default()),
            mailer::Outcome::Retry(error) | mailer::Outcome::Failed(error) => println!("Failed for {}: {error}", delivery.subscriber.email),
        }
    }

    println!("Done — check {to_email}");

    Ok(())
}
