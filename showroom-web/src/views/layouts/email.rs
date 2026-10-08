use boutique::html;
use maud::{DOCTYPE, Markup, PreEscaped};
use serde::Serialize;

use crate::{renderer::email::{EmailBlock, caption_style}, theme::{Color, EmailTheme, Layout}};

#[allow(dead_code)]
pub enum Align {
    Left,
    Right,
    Center,
    Justify,
}

impl Align {
    fn as_str(&self) -> &'static str {
        match self {
            Align::Left     => "left",
            Align::Right    => "right",
            Align::Center   => "center",
            Align::Justify  => "justify",
        }
    }
}

pub fn base_email_layout(title: &str, preheader: Option<&str>, theme: EmailTheme, content: Markup) -> Markup {

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
            }
            body style=(format!("margin:0;padding:0;background-color:{};", theme.surface)) {
                @if let Some(pre) = preheader {
                    div style="display:none;max-height:0;overflow:hidden;mso-hide:all;" {
                        (pre)
                        (PreEscaped("&zwnj;&nbsp;".repeat(75)))
                    }
                }


                table role="presentation" cellpadding="0" cellspacing="0" border="0" width="100%" style=(format!("background-color:{};", theme.surface)) {
                    tr {
                        td align="center" style="padding:40px 20px;" {
                            table role="presentation" cellpadding="0" cellspacing="0" border="0" width="600" style=(format!("max-width:600px;width:100%;background-color:{};", theme.surface)) {
                                (content)
                            }
                        }
                    }
                }
            }
        }
    }
}

const GUTTER: &str = "40";

pub fn email_section(content: &str, padding_top: &str, padding_bottom: &str, align: Option<Align>, theme: &EmailTheme) -> Markup {
    let mut style = format!(
        "padding-top:{padding_top};padding-bottom:{padding_bottom};font-family:{};font-size:14px;color:{};",
        theme.font_body,
        theme.text
    );
    if let Some(a) = align {
        style.push_str(&format!("text-align:{};", a.as_str()));
    }

    html! {
        tr {
            td width=(GUTTER) style=(format!("width:{GUTTER}px;padding:0;")) {}
            td style=(style) {
                (PreEscaped(content))
            }
            td width=(GUTTER) style=(format!("width:{GUTTER}px;padding:0;")) {}
        }
    }
}

pub fn email_p(content: Markup, theme: &EmailTheme, color_override: Option<Color>) -> Markup {
    let style = format!(
        "margin:0 0 16px;font-family:{};font-size:14px;line-height:1.5;color:{};",
        theme.font_body,
        color_override.unwrap_or(theme.text)
    );

    html! {
        p style=(style) { (content) }
    }
}

pub fn email_a(content: Markup, theme: &EmailTheme, href: &str, color_override: Option<Color>) -> Markup {
    let style = format!(
        "font-family:{};color:{};text-decoration:underline;",
        theme.font_body,
        color_override.unwrap_or(theme.link)
    );

    html! {
        a href=(href) style=(style) { (content) }
    }
}

fn full_width_image_section(src: &str, caption: Option<&str>, theme: &EmailTheme) -> Markup {
    html! {
        tr {
            td colspan="3" style="padding:0;" width="600" {
                img src=(src) alt="" width="600" style="width:100%;max-width:100%;height:auto;display:block;";
            }
        }
        @if let Some(caption) = caption {
            tr {
                td width=(GUTTER) style=(format!("width:{GUTTER}px;padding:0;")) {}
                td style="padding:0 0 24px 0;" {
                    p style=(caption_style(theme)) { (PreEscaped(caption)) }
                }
                td width=(GUTTER) style=(format!("width:{GUTTER}px;padding:0;")) {}
            }
        }
    }
}

pub fn email_button(label: &str, href: &str, theme: &EmailTheme) -> Markup {
    let td_style = format!("border-radius:4px;background-color:{};", theme.text);
    let a_style = format!(
        "display:inline-block;padding:12px 24px;font-family:{};font-size:14px;font-weight:600;color:{};text-decoration:none;",
        theme.font_body,
        theme.surface
    );

    html! {
        table role="presentation" cellpadding="0" cellspacing="0" border="0" {
            tr {
                td style=(td_style) {
                    a href=(href) style=(a_style) { (label) }
                }
            }
        }
    }
}

pub fn confirmation_html(name: Option<&str>, confirm_url: &str, publication_name: &str, theme: EmailTheme) -> Markup {

    let content = html! {
        @if let Some(n) = name {
            (email_p(html!{ "Hi " (n) ","}, &theme, None))
        }
        (email_p(
            html!{ "Please confirm your subscription to " strong { (publication_name) } "."},
            &theme, None)
        )
    };

    let button = html! {
        (email_button("Confirm subscription", confirm_url, &theme))
    };

    let footer = html! {
        (email_p(
            html! { "If you did not request this, you can safely ignore this email." },
            &theme, Some(theme.muted)
        ))
    };

    base_email_layout(
        "Confirm your subscription",
        Some("Please confirm your subscription."),
        theme,
        html! {
            (email_section(&content.0, "32px", "32px", None, &theme))
            (email_section(&button.0, "32px", "32px", Some(Align::Center), &theme))
            (email_section(&footer.0, "32px", "32px", None, &theme))
        }
    )
}

#[derive(Serialize, Default)]
pub struct NewsletterTemplateData {
    pub greeting_html: String,
    pub unsubscribe_url: String,
}


pub fn generate_subscriber_data(name: Option<&str>, unsubscribe_url: &str) -> NewsletterTemplateData {
    let greeting_html = name.map(|n| format!("Hi {n},")).unwrap_or_default();

    NewsletterTemplateData {
        greeting_html,
        unsubscribe_url: unsubscribe_url.to_string(),
    }
}

pub fn newsletter_template(
    title: &str,
    subtitle: Option<&str>,
    publication_name: &str,
    date: &str,
    read_online_url: &str,
    publication_url: &str,
    theme: EmailTheme,
    mut content: Vec<EmailBlock>,
) -> Markup {

    match content.first_mut() {
        Some(EmailBlock::Content(s)) => *s = "{{greeting_html}}".to_string() + s,
        _ => content.insert(0, EmailBlock::Content("{{greeting_html}}".to_string())),
    }

    let letter = theme.layout == Layout::Letter;
    let centred = theme.layout == Layout::Centred;
    let (header_align, info_align) = match theme.layout {
        Layout::Default => (Align::Right, None),
        Layout::Compact => (Align::Right, Some(Align::Right)),
        Layout::Centred => (Align::Center, Some(Align::Center)),
        Layout::Letter => (Align::Right, None),
    };

    let header = html! {
        (email_p(html! {
            @if !letter {
                (date) " · "
            }
            (email_a(
                html! { "Read in browser" },
                &theme,
                &read_online_url,
                Some(theme.muted)
            ))
        }, &theme, Some(theme.muted)))
    };

    let info = html! {
        h1 style=(format!(
            "margin:24px 0 8px;font-family:{};font-size:{}px;font-weight:bold;color:{};line-height:1.2;",
            theme.font_title,
            theme.title_px,
            theme.text
        )) {
            (title)
        }
        @if let Some(sub) = subtitle {
            (email_p(
                html!{ (sub) },
                &theme,
                Some(theme.muted)
            ))
        }
        @if !letter {
            (email_p (
                email_a(
                    html! { (publication_name) },
                    &theme,
                    &publication_url,
                    None),
                &theme,
                None
            ))
        }
        @if centred {
            div style=(format!("width:64px;height:1px;margin:24px auto 0;background-color:{};font-size:0;line-height:0;", theme.text)) {}
        }
    };

    let content_rows = html! {
        @for block in &content {
            @match block {
                EmailBlock::Content(s) => (email_section(s, "0", "32px", None, &theme)),
                EmailBlock::FullWidthImage { src, caption } => (full_width_image_section(src, caption.as_deref(), &theme)),
            }
        }
    };

    let footer = html! {
        div  {
            (email_a(
                html! { "Unsubscribe from "(publication_name)"." },
                &theme,
                "{{unsubscribe_url}}",
                None
            ))
        }
        div {
            "This newsletter is powered by "
            (email_a(
                html! { "Showroom" },
                &theme,
                "https://show.room.lc",
                None
            ))
            "."
        }
    };

    base_email_layout(
        title,
        subtitle,
        theme,
        html! {
            (email_section(&header.0, "32px", "24px", Some(header_align), &theme))
            (email_section(&info.0, "0", "0", info_align, &theme))
            (content_rows)
            (email_section(&footer.0, "24px", "24px", Some(Align::Right), &theme))
        }
    )
}

pub fn confirmation_text(name: Option<&str>, confirm_url: &str, publication_name: &str) -> String {
    let greeting = name.map(|n| format!("Hi {n},\n\n")).unwrap_or_default();
    format!("{greeting}Please confirm your subscription to {publication_name} by visiting:\n\n{confirm_url}\n\nIf you did not request this, you can safely ignore this email.")
}
