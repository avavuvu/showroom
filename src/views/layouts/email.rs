use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::renderer::html::ThemeVariables;

pub fn email_layout(title: &str, theme: Option<ThemeVariables>, content: Markup) -> Markup {
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
                table role="presentation" cellpadding="0" cellspacing="0" border="0" width="100%" style=(format!("background-color:{};", theme.color_surface)) {
                    tr {
                        td align="center" style="padding:40px 20px;" {
                            table role="presentation" cellpadding="0" cellspacing="0" border="0" width="600" style=(format!("max-width:600px;width:100%;background-color:{};", theme.color_surface)) {
                                (email_section("{{greeting}}", "padding:32px 40px 0", &theme))
                                (email_section(&content.0, "padding:24px 40px 32px;", &theme))
                                (email_section("{{unsubscribe}}", "padding:24px 40px;", &theme))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn email_section(content: &str, padding: &str, theme: &ThemeVariables) -> Markup {
    let style = format!(
        "padding:{padding};font-family:{};font-size:14px;color:{};",
        theme.font_body, theme.color_text
    );

    html! {
        tr {
            td style=(style) {
                (PreEscaped(content))
            }
        }
    }
}

pub fn email_p(content: Markup, theme: &ThemeVariables, text_color: Option<&str>) -> Markup {
    let style = format!(
        "margin:0 0 16px;font-family:{};font-size:14px;line-height:1.5;color:{};",
        theme.font_body, text_color.unwrap_or(&theme.color_text)
    );

    html! {
        p style=(style) { (content) }
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

    email_layout(
        "Confirm your subscription",
        Some(theme.clone()),
        html! {
            @if let Some(n) = name {
                (email_p(html!{ "Hi " (n) ","}, &theme, None))
            }

            (email_p(
                html!{ "Please confirm your subscription to " strong { "@"(handle) } "."},
                &theme, None)
            )

            (email_button("Confirm subscription", confirm_url, &theme))

            (email_p(
                html! { "If you did not request this, you can safely ignore this email." },
                &theme, Some(&theme.color_muted)
            ))
        }
    )
}


pub fn confirmation_text(name: Option<&str>, confirm_url: &str, handle: &str) -> String {
    let greeting = name.map(|n| format!("Hi {n},\n\n")).unwrap_or_default();
    format!("{greeting}Please confirm your subscription to {handle} by visiting:\n\n{confirm_url}\n\nIf you did not request this, you can safely ignore this email.")
}
