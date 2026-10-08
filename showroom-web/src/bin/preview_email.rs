use chrono::Utc;
use serde_json::json;
use showroom_web::{
    document::Document,
    renderer::email::render_email,
    theme::Theme,
    views::layouts::newsletter_template,
};

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

    let html = newsletter_template(
        "My Test Newsletter",
        Some("Testing the mailer"),
        "test",
        &date,
        "http://test.showroom.you:3000/my-test-newsletter",
        "http://test.showroom.you:3000",
        theme,
        rendered_content,
    );

    let preview = html.0
        .replace("{{greeting_html}}", "Hi Ava,")
        .replace("{{unsubscribe_url}}", "#");

    let path = std::env::temp_dir().join("email_preview.html");
    std::fs::write(&path, &preview)?;
    std::process::Command::new("open").arg(&path).spawn()?;

    Ok(())
}
