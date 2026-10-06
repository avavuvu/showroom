use boutique::html;
use maud::Markup;
use crate::models::newsletter;
use crate::components::{Button, Editor, SaveStatus};
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {

    let back_url = ctx.dashboard_url();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        html! { Button(href = format!("{}/{}", ctx.publication_url(), newsletter.slug)) .primary { "View" } }
    } else {
        html! { Button(href = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)) .primary { "Publish" } }
    };

    base(
        &page("Edit").htmx(),
        html! {
        div.edit-view {
            @if newsletter.sent_at.is_some() {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header {
                div.left {
                    Button(href = back_url) .secondary { "Back" }

                    SaveStatus;

                }

                div {
                    (view_or_preview_button)
                }
            }

            Editor(&newsletter.id);
        }
    })
}
