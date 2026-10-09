use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::{models::publication::EMAIL_PICTURE_SIZE, theme::EmailTheme};

const GUTTER: &str = "32";
const STYLESHEET: &str = include_str!("email.css");

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Center,
    Justify,
}

impl Align {
    fn as_str(self) -> &'static str {
        match self {
            Align::Left     => "left",
            Align::Right    => "right",
            Align::Center   => "center",
            Align::Justify  => "justify",
        }
    }
}

pub fn stylesheet(theme: &EmailTheme) -> String {
    let variables = [
        ("surface", theme.surface.to_string()),
        ("text", theme.text.to_string()),
        ("link", theme.link.to_string()),
        ("muted", theme.muted.to_string()),
        ("font-body", theme.font_body.to_string()),
        ("font-title", theme.font_title.to_string()),
        ("text-size", format!("{}px", theme.text_px)),
        ("caption-size", format!("{}px", theme.text_px.saturating_sub(2))),
        ("title-size", format!("{}px", theme.title_px)),
        ("gutter", format!("{GUTTER}px")),
        ("picture-size", format!("{EMAIL_PICTURE_SIZE}px")),
    ];

    let css = variables
        .iter()
        .fold(STYLESHEET.to_string(), |css, (name, value)| css.replace(&format!("var(--{name})"), value));

    if css.contains("var(") {
        eprintln!("[email] email.css uses a variable that stylesheet() does not set");
        debug_assert!(false, "email.css uses a variable that stylesheet() does not set");
    }

    css
}

#[component]
pub fn section(
    #[builder(default = "0")] top: &str,
    #[builder(default = "0")] bottom: &str,
    align: Option<Align>,
    children: Markup,
) -> Markup {
    let mut style = format!("padding-top:{top};padding-bottom:{bottom};");
    if let Some(align) = align {
        style.push_str(&format!("text-align:{};", align.as_str()));
    }

    html! {
        tr {
            td.email-gutter width=(GUTTER) {}
            td.email-section style=(style) { (children) }
            td.email-gutter width=(GUTTER) {}
        }
    }
}

#[component]
pub fn button(#[builder(start_fn)] href: &str, children: Markup) -> Markup {
    html! {
        table role="presentation" cellpadding="0" cellspacing="0" border="0" {
            tr {
                td.email-button-cell {
                    a.email-button href=(href) { (children) }
                }
            }
        }
    }
}

#[component]
pub fn full_width_image(#[builder(start_fn)] src: &str, #[builder(required)] caption: Option<&str>) -> Markup {
    html! {
        tr {
            td.email-full-width colspan="3" width="600" {
                img.email-full-width-image src=(src) alt="" width="600";
            }
        }
        @if let Some(caption) = caption {
            tr {
                td.email-gutter width=(GUTTER) {}
                td.email-caption-cell {
                    p.email-caption { (maud::PreEscaped(caption)) }
                }
                td.email-gutter width=(GUTTER) {}
            }
        }
    }
}
