use chrono::Utc;
use dotenvy::dotenv;
use serde_json::json;
use showroom_web::{
    mailer,
    models::{newsletter::Model as Newsletter, subscriber::Model as Subscriber},
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

    let newsletter = Newsletter {
        id: "test-05".to_string(),
        user_id: "test-user".to_string(),
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
        rendered: None,
        sent_at: None,
        created_at: Utc::now().fixed_offset(),
        updated_at: Utc::now().fixed_offset(),
    };

    let subscribers = vec![Subscriber {
        token: "test-token-02".to_string(),
        user_id: "test-users".to_string(),
        name: Some("Ava".to_string()),
        email: to_email.clone(),
        is_confirmed: true,
        created_at: Utc::now().fixed_offset(),
    }];

    println!("Sending test newsletter to {to_email}...");
    println!("Creating SES template...");

    mailer::send_newsletter(&client, &newsletter, "test", &subscribers, &urls).await?;

    println!("Done — check {to_email}");

    Ok(())
}
