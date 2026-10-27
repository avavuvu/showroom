use boutique::html;
use maud::{Markup, PreEscaped};

use crate::components::{EditorPage, EditorProps};
use crate::document::Document;
use crate::models::review::ReviewSummary;
use crate::models::newsletter;
use crate::views::context::PageContext;
use crate::views::layouts::{base, installable, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model, reviews: Vec<ReviewSummary>) -> Markup {
    let sent = newsletter.published_at.is_some();
    let props = EditorProps {
        id: &newsletter.id,
        slug: &ctx.publication().slug,
        reviews,
        revision: newsletter.revision,
        content: Document::from_stored(&newsletter.content),
    };
    let next = if sent {
        format!("{}/{}", ctx.publication_url(), newsletter.slug)
    } else {
        format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)
    };

    base(
        &installable(page("Edit").entry_css("editor").htmx()).viewport("width=device-width, initial-scale=1, interactive-widget=resizes-content"),
        html! {
            @if let Some(publication) = &ctx.publication {
                style { (PreEscaped(publication.theme().css())) }
            }
            EditorPage(
                props = &props,
                title = &newsletter.title,
                subtitle = [newsletter.subtitle.as_deref()],
                back = &ctx.dashboard_url(),
                next = &next,
                sent = sent,
            );
        },
    )
}
