use maud::{Markup, html};
use crate::views::context::PageContext;

pub fn footer(ctx: &PageContext) -> Markup {
    html! {
        footer.footer-full {
            div.logo-container {
                img.wordmark src="/icons/wordmark.svg" alt="Showroom";
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
