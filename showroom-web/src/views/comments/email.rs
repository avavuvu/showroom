use boutique::html;
use maud::Markup;

use crate::{
    components::email::{Align, Button, Section},
    services::comments::Content,
    theme::EmailTheme,
    views::layouts::base_email_layout,
};

pub struct CommentConfirmation<'a> {
    pub confirm_url: &'a str,
    pub publication_name: &'a str,
    pub newsletter_title: &'a str,
    pub comment: &'a Content,
    pub subscribes: bool,
}

fn confirmation_action(confirmation: &CommentConfirmation) -> &'static str {
    if confirmation.subscribes { "Subscribe and publish" } else { "Publish comment" }
}

pub fn comment_confirmation_html(confirmation: &CommentConfirmation, theme: EmailTheme) -> Markup {
    base_email_layout(
        "Publish your comment",
        Some("Confirm your comment to publish it."),
        theme,
        html! {
            Section(top = "32px", bottom = "32px") {
                blockquote.email-quote { (confirmation.comment.email_html()) }

                p { "Would you like to leave this comment on " strong { (confirmation.newsletter_title) } " by " (confirmation.publication_name) "?" }
                @if confirmation.subscribes {
                    p { "Commenting will also subscribe you to " strong { (confirmation.publication_name) } "." }
                }
            }
            Section(top = "32px", bottom = "32px", align = Align::Center) {
                Button(confirmation.confirm_url) { (confirmation_action(confirmation)) }
            }
            Section(top = "32px", bottom = "32px") {
                p.muted { "If this wasn't you, you can ignore this email." }
            }
        }
    )
}

pub fn comment_confirmation_text(confirmation: &CommentConfirmation) -> String {
    let CommentConfirmation { confirm_url, publication_name, newsletter_title, comment, subscribes } = confirmation;
    let quote = comment.email_text().lines().map(|line| if line.is_empty() { ">".to_string() } else { format!("> {line}") }).collect::<Vec<_>>().join("\n");
    let subscription = if *subscribes { format!("Commenting will also subscribe you to {publication_name}.\n\n") } else { String::new() };
    let action = confirmation_action(confirmation);
    format!(
        "{quote}\n\nWould you like to leave this comment on {newsletter_title} by {publication_name}?\n\n{subscription}{action}: {confirm_url}\n\nIf this wasn't you, you can ignore this email."
    )
}
