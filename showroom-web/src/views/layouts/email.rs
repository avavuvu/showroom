use boutique::html;
use bq_components::component;
use maud::{DOCTYPE, Markup, PreEscaped};
use serde::Serialize;

use crate::{
    components::email::{Align, Button, FullWidthImage, Section, stylesheet},
    models::publication::EMAIL_PICTURE_SIZE,
    renderer::{email::EmailBlock, greeting},
    theme::{EmailTheme, Layout},
};

pub fn base_email_layout(title: &str, preheader: Option<&str>, theme: EmailTheme, content: Markup) -> Markup {
    let document = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                style { (PreEscaped(stylesheet(&theme))) }
            }
            body {
                @if let Some(pre) = preheader {
                    div.email-preheader {
                        (pre)
                        (PreEscaped("&zwnj;&nbsp;".repeat(75)))
                    }
                }

                table.email-outer role="presentation" cellpadding="0" cellspacing="0" border="0" width="100%" {
                    tr {
                        td.email-frame align="center" {
                            table.email-inner role="presentation" cellpadding="0" cellspacing="0" border="0" width="600" {
                                (content)
                            }
                        }
                    }
                }
            }
        }
    };

    inline_css(document.into_string())
}

fn inline_css(document: String) -> Markup {
    let inliner = css_inline::CSSInliner::options().keep_at_rules(true).build();
    match inliner.inline(&document) {
        Ok(inlined) => PreEscaped(inlined),
        Err(e) => {
            eprintln!("[email] css inlining failed, sending with a style tag: {e}");
            PreEscaped(document)
        }
    }
}

pub fn confirmation_html(name: Option<&str>, confirm_url: &str, publication_name: &str, theme: EmailTheme) -> Markup {
    base_email_layout(
        "Confirm your subscription",
        Some("Please confirm your subscription."),
        theme,
        html! {
            Section(top = "32px", bottom = "32px") {
                @if let Some(n) = name {
                    p { "Hi " (n) "," }
                }
                p { "Please confirm your subscription to " strong { (publication_name) } "." }
            }
            Section(top = "32px", bottom = "32px", align = Align::Center) {
                Button(confirm_url) { "Confirm subscription" }
            }
            Section(top = "32px", bottom = "32px") {
                p.muted { "If you did not request this, you can safely ignore this email." }
            }
        }
    )
}

#[derive(Serialize, Default)]
pub struct NewsletterTemplateData {
    pub greeting: String,
    pub unsubscribe_url: String,
}

pub fn generate_subscriber_data(greeting_template: &str, name: Option<&str>, unsubscribe_url: &str) -> NewsletterTemplateData {
    let greeting = greeting::for_subscriber(greeting_template, name)
        .map(|text| html! { (text) }.into_string())
        .unwrap_or_default();

    NewsletterTemplateData {
        greeting,
        unsubscribe_url: unsubscribe_url.to_string(),
    }
}

#[component]
pub fn newsletter_body(
    title: &str,
    subtitle: Option<&str>,
    publication_name: &str,
    date: &str,
    read_online_url: &str,
    publication_url: &str,
    picture: &str,
    layout: Layout,
    greeting: Option<Markup>,
    content: &[EmailBlock],
) -> Markup {
    let letter = layout == Layout::Letter;
    let centred = layout == Layout::Centred;
    let (header_align, info_align) = match layout {
        Layout::Default => (Align::Right, None),
        Layout::Compact => (Align::Right, Some(Align::Right)),
        Layout::Centred => (Align::Center, Some(Align::Center)),
        Layout::Letter => (Align::Right, None),
    };

    html! {
        Section(top = "32px", bottom = "24px", align = header_align) {
            p.muted.small {
                @if !letter {
                    (date) " · "
                }
                a.muted href=(read_online_url) { "Read in browser" }
            }
        }
        Section(top = "8px", bottom = "16px", maybe_align = info_align) {
            img.email-picture src=(picture) width=(EMAIL_PICTURE_SIZE) height=(EMAIL_PICTURE_SIZE) alt=(publication_name);
            h1.email-title { (title) }
            @if let Some(sub) = subtitle {
                p.muted { (sub) }
            }
            @if !letter {
                p {
                    a href=(publication_url) { (publication_name) }
                }
            }
            @if centred {
                div.email-rule {}
            }
        }
        @if let Some(greeting) = greeting {
            Section { (greeting) }
        }
        @for block in content {
            @match block {
                EmailBlock::Content(s) => {
                    Section(bottom = "32px") { (PreEscaped(s)) }
                }
                EmailBlock::FullWidthImage { src, caption } => {
                    FullWidthImage(src, caption = caption.as_deref());
                }
            }
        }
        Section(top = "24px", bottom = "24px", align = Align::Right) {
            div {
                a href="{{unsubscribe_url}}" { "Unsubscribe from " (publication_name) "." }
            }
            div {
                "This newsletter is powered by "
                a href="https://show.room.lc" { "Showroom" }
                "."
            }
        }
    }
}

pub fn confirmation_text(name: Option<&str>, confirm_url: &str, publication_name: &str) -> String {
    let greeting = name.map(|n| format!("Hi {n},\n\n")).unwrap_or_default();
    format!("{greeting}Please confirm your subscription to {publication_name} by visiting:\n\n{confirm_url}\n\nIf you did not request this, you can safely ignore this email.")
}
