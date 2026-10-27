use boutique::html;
use maud::{Markup, PreEscaped};

use crate::components::{ReviewPage, ReviewProps, Workspace};
use crate::document::Document;
use crate::models::{newsletter, publication, review, review_link};
use crate::views::layouts::{base, page};

pub fn review(publication: &publication::Model, newsletter: &newsletter::Model, review: &review::Model, link: &review_link::Model, url: &str) -> Markup {
    let base_document = Document::from_stored(&review.base);
    let props = ReviewProps {
        url,
        revision: link.revision,
        section_id: review.section_id.as_deref(),
        content: link.proposed.as_ref().map(Document::from_stored).unwrap_or_else(|| base_document.clone()),
        base: base_document,
        sent: link.sent_at.is_some(),
    };
    let title = newsletter.display_title();

    base(
        &page(title).entry_css("editor").viewport("width=device-width, initial-scale=1, interactive-widget=resizes-content"),
        html! {
            style { (PreEscaped(publication.theme().css())) }
            ReviewPage(
                props = &props,
                publication = &publication.name,
                newsletter = title,
                note = [review.note.as_deref()],
                reply = [link.reply.as_deref()],
                sent_at = [link.sent_at.as_ref()],
            );
        },
    )
}

pub fn inactive() -> Markup {
    base(
        &page("Link inactive"),
        html! {
            Workspace(left = html! {}, right = html! {}) .review-workspace {
                main.workspace-body.prose {
                    div.review-inactive.flow {
                        h1 { "This link isn\u{2019}t active" }
                        p { "It may have expired or been deleted. Ask the person who sent it for a new link." }
                    }
                }
            }
        },
    )
}
