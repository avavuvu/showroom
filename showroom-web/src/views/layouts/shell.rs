use maud::{Markup, html};
use crate::views::{components::layout::{footer::footer, header::header}, context::PageContext, layouts::{Head, base}};

pub fn shell(view: Head, page: &PageContext, content: Markup) -> Markup {
    base(
        &view,
        html! {
            (header(page))
            (content)
            (footer(page))
        }
    )
}
