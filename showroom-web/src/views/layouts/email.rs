use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde::Serialize;

use crate::renderer::email::{EmailBlock, ThemeVariables};

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

pub fn base_email_layout(title: &str, preheader: Option<&str>, theme: Option<ThemeVariables>, content: Markup) -> Markup {
    let theme = theme.unwrap_or_default();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
            }
            body style=(format!("margin:0;padding:0;background-color:{};", theme.color_surface)) {
                @if let Some(pre) = preheader {
                    div style="display:none;max-height:0;overflow:hidden;mso-hide:all;" {
                        (pre)
                        (PreEscaped("&zwnj;&nbsp;".repeat(75)))
                    }
                }


                table role="presentation" cellpadding="0" cellspacing="0" border="0" width="100%" style=(format!("background-color:{};", theme.color_surface)) {
                    tr {
                        td align="center" style="padding:40px 20px;" {
                            table role="presentation" cellpadding="0" cellspacing="0" border="0" width="600" style=(format!("max-width:600px;width:100%;background-color:{};", theme.color_surface)) {
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

pub fn email_section(content: &str, padding_top: &str, padding_bottom: &str, align: Option<Align>, theme: &ThemeVariables) -> Markup {
    let mut style = format!(
        "padding-top:{padding_top};padding-bottom:{padding_bottom};font-family:{};font-size:14px;color:{};",
        theme.font_body, theme.color_text
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

pub fn email_p(content: Markup, theme: &ThemeVariables, color_override: Option<&str>) -> Markup {
    let style = format!(
        "margin:0 0 16px;font-family:{};font-size:14px;line-height:1.5;color:{};",
        theme.font_body, color_override.unwrap_or(&theme.color_text)
    );

    html! {
        p style=(style) { (content) }
    }
}

pub fn email_a(content: Markup, theme: &ThemeVariables, href: &str, color_override: Option<&str>) -> Markup {
    let style = format!(
        "font-family:{};color:{};text-decoration:underline;",
        theme.font_body, color_override.unwrap_or(&theme.color_primary)
    );

    html! {
        a href=(href) style=(style) { (content) }
    }
}

fn full_width_image_section(src: &str, alt: &str) -> Markup {
    html! {
        tr {
            td colspan="3" style="padding:0;" width="600" {
                img src=(src) alt=(alt) width="600" style="width:100%;max-width:100%;height:auto;display:block;";
            }
        }
    }
}

pub fn email_button(label: &str, href: &str, theme: &ThemeVariables) -> Markup {
    let td_style = format!("border-radius:4px;background-color:{};", theme.color_text);
    let a_style = format!(
        "display:inline-block;padding:12px 24px;font-family:{};font-size:14px;font-weight:600;color:#ffffff;text-decoration:none;",
        theme.font_body
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

pub fn confirmation_html(name: Option<&str>, confirm_url: &str, handle: &str) -> Markup {
    let theme = ThemeVariables::default();

    let content = html! {
        @if let Some(n) = name {
            (email_p(html!{ "Hi " (n) ","}, &theme, None))
        }
        (email_p(
            html!{ "Please confirm your subscription to " strong { "@"(handle) } "."},
            &theme, None)
        )
    };

    let button = html! {
        (email_button("Confirm subscription", confirm_url, &theme))
    };

    let footer = html! {
        (email_p(
            html! { "If you did not request this, you can safely ignore this email." },
            &theme, Some(&theme.color_muted)
        ))
    };

    base_email_layout(
        "Confirm your subscription",
        Some("Please confirm your subscription."),
        Some(theme.clone()),
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
    handle: &str,
    date: &str,
    read_online_url: &str,
    user_url: &str,
    theme: Option<ThemeVariables>,
    mut content: Vec<EmailBlock>,
) -> Markup {
    let theme = theme.unwrap_or_default();

    match content.first_mut() {
        Some(EmailBlock::Content(s)) => *s = "{{greeting_html}}".to_string() + s,
        _ => content.insert(0, EmailBlock::Content("{{greeting_html}}".to_string())),
    }

    let header = html! {
        (email_p(html! {
            (date) " · "
            (email_a(
                html! { "Read in browser" },
                &theme,
                &read_online_url,
                Some(&theme.color_muted)
            ))
        }, &theme, Some(&theme.color_muted)))
    };

    let info = html! {
        h1 style=(format!(
            "margin:24px 0 8px;font-family:{};font-size:28px;font-weight:bold;color:{};line-height:1.2;",
            theme.font_title, theme.color_text
        )) {
            (title)
        }
        @if let Some(sub) = subtitle {
            (email_p(
                html!{ (sub) },
                &theme,
                Some(&theme.color_muted)
            ))
        }
        (email_p (
            email_a(
                html! { "@"(handle) },
                &theme,
                &user_url,
                None),
            &theme,
            None
        ))

    };

    let content_rows = html! {
        @for block in &content {
            @match block {
                EmailBlock::Content(s) => (email_section(s, "0", "32px", None, &theme)),
                EmailBlock::FullWidthImage { src, alt } => (full_width_image_section(src, alt)),
            }
        }
    };

    let footer = html! {
        div  {
            (email_a(
                html! { "Unsubscribe from @"(handle)"." },
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
        Some(theme.clone()),
        html! {
            (email_section(&header.0, "32px", "24px", Some(Align::Right), &theme))
            (email_section(&info.0, "0", "0", None, &theme))
            (content_rows)
            (email_section(&footer.0, "24px", "24px", Some(Align::Right), &theme))
        }
    )
}

pub fn confirmation_text(name: Option<&str>, confirm_url: &str, handle: &str) -> String {
    let greeting = name.map(|n| format!("Hi {n},\n\n")).unwrap_or_default();
    format!("{greeting}Please confirm your subscription to {handle} by visiting:\n\n{confirm_url}\n\nIf you did not request this, you can safely ignore this email.")
}
