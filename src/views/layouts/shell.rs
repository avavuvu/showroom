use maud::{Markup, html};
use crate::views::{components::layout::{footer::footer, header::header}, context::PageContext, layouts::base};

pub fn shell(ctx: &PageContext, content: Markup) -> Markup {
    base(html! {
        (header(ctx))
        (content)
        (footer(ctx))
    })
}
