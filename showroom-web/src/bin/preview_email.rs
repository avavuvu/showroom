use chrono::Utc;
use serde_json::json;
use showroom_web::{
    document::Document,
    renderer::{email::render_email, greeting},
    theme::Theme,
    views::layouts::{NewsletterBody, base_email_layout},
};
use boutique::html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = json!({
        "type": "doc",
        "content": [
            {
                "type": "heading",
                "attrs": { "level": 2 },
                "content": [{ "type": "text", "text": "A heading" }]
            },
            {
                "type": "paragraph",
                "content": [{ "type": "text", "text": "This is a preview of the newsletter email layout." }]
            },
            {
                "type": "paragraph",
                "content": [
                    { "type": "text", "text": "Bold text", "marks": [{ "type": "bold" }] },
                    { "type": "text", "text": " and " },
                    { "type": "text", "text": "italic text", "marks": [{ "type": "italic" }] },
                    { "type": "text", "text": "." }
                ]
            }
        ]
    });

    let theme = Theme::default().email();
    let rendered_content = render_email(&Document::from_value(&content)?, theme);
    let date = Utc::now().format("%B %-d, %Y").to_string();

    let greeting = greeting::for_subscriber(greeting::DEFAULT, Some("Ava")).map(|text| html! { p { (text) } });

    let html = base_email_layout(
        "My Test Newsletter",
        Some("Testing the mailer"),
        theme,
        html! {
            NewsletterBody(
                title = "My Test Newsletter",
                subtitle = "Testing the mailer",
                publication_name = "test",
                date = &date,
                read_online_url = "http://test.showroom.you:3000/my-test-newsletter",
                publication_url = "http://test.showroom.you:3000",
                picture = "https://show.room.lc/avatars/0.png",
                layout = theme.layout,
                maybe_greeting = greeting,
                content = &rendered_content,
            );
        },
    );

    let preview = html.0.replace("{{unsubscribe_url}}", "#");

    let path = std::env::temp_dir().join("email_preview.html");
    std::fs::write(&path, &preview)?;
    std::process::Command::new("open").arg(&path).spawn()?;

    Ok(())
}
