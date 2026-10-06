use maud::{Markup, html};
use crate::models::newsletter;
use crate::components::button;
use crate::components::{editor, save_status};
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {

    let back_url = ctx.dashboard_url();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        button(html!("View")).href(format!("{}/{}", ctx.publication_url(), newsletter.slug)).primary()
    } else {
        button(html!("Publish")).href(format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)).primary()
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
                    (button(html!("Back")).href(back_url).secondary())

                    (save_status())

                }

                div {
                    (view_or_preview_button)
                }
            }

            (editor(&newsletter.id))
        }
    })
}
