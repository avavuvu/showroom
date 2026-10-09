use boutique::html;
use maud::Markup;
use crate::{components::Wordmark, views::context::PageContext};

pub fn footer(ctx: &PageContext) -> Markup {
    html! {
        footer.footer-full {
            div.logo-container {
                Wordmark;
            }
            div.content {
                p {
                    a href=(ctx.urls.base()) { "Showroom" }
                    em { "Newsletters for people like you" }
                    a href=(format!("{}/about", ctx.urls.base())) { "Find out more" }
                }
            }
        }
    }
}
