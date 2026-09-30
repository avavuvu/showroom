use maud::{Markup, PreEscaped, html};
use crate::views::{components::layout::{footer::footer, header::header}, context::PageContext, layouts::{Head, base}};

pub fn shell(view: Head, page: &PageContext, content: Markup) -> Markup {
    base(
        &view,
        html! {
            @if let Some(publication) = &page.publication {
                style { (PreEscaped(publication.theme().css())) }
            }
            (header(page))
            (content)
            (footer(page))
        }
    )
}
