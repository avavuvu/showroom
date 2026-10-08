use boutique::html;
use maud::{Markup, PreEscaped};
use serde_json::json;

use crate::components::editor::NewsletterEditor;
use crate::components::{Button, Editor, SaveStatus};
use crate::document::Document;
use crate::models::newsletter;
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let back_url = ctx.dashboard_url();
    let sent = newsletter.sent_at.is_some();
    let props = json!({
        "id": newsletter.id,
        "revision": newsletter.revision,
        "content": Document::from_stored(&newsletter.content),
    })
    .to_string();

    base(
        &page("Edit").htmx(),
        html! {
        @if let Some(publication) = &ctx.publication {
            style { (PreEscaped(publication.theme().css())) }
        }
        div.edit-view bq-setup=(NewsletterEditor) data-props=(props) {
            @if sent {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header {
                div.left {
                    Button(href = back_url) .secondary bq-ref="leave" { "Back" }
                    SaveStatus;
                }

                div {
                    @if sent {
                        Button(href = format!("{}/{}", ctx.publication_url(), newsletter.slug)) .primary bq-ref="leave" { "View" }
                    } @else {
                        Button(href = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)) .primary bq-ref="leave" { "Publish" }
                    }
                }
            }

            Editor(title = &newsletter.title, subtitle = newsletter.subtitle.as_deref());
        }
    })
}
