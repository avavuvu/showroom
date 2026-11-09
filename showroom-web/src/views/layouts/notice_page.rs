use boutique::html;
use maud::Markup;

use crate::views::{PageContext, layouts::{page, shell}};

pub fn notice_page(ctx: &PageContext, title: &str, content: Markup) -> Markup {
    shell(page(title), ctx, html! {
        main.article-layout {
            div.notice-page.flow {
                h1 { (title) }
                (content)
            }
        }
    })
}
