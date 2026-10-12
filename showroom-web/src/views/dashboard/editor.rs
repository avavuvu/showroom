use boutique::html;
use maud::{Markup, PreEscaped};
use serde_json::json;

use crate::components::EditorPage;
use crate::document::Document;
use crate::models::newsletter;
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let sent = newsletter.published_at.is_some();
    let props = json!({
        "id": newsletter.id,
        "revision": newsletter.revision,
        "content": Document::from_stored(&newsletter.content),
    })
    .to_string();
    let next = if sent {
        format!("{}/{}", ctx.publication_url(), newsletter.slug)
    } else {
        format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)
    };

    base(
        &page("Edit").htmx(),
        html! {
            @if let Some(publication) = &ctx.publication {
                style { (PreEscaped(publication.theme().css())) }
            }
            EditorPage(
                props = &props,
                title = &newsletter.title,
                subtitle = newsletter.subtitle.as_deref(),
                back = &ctx.dashboard_url(),
                next = &next,
                sent = sent,
            );
        },
    )
}
