use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ALPINE_ENTRY, ISLANDS_ENTRY, base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    let back_url = ctx.dashboard_url();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        let view_url = format!("{}/{}", ctx.publication_url(), newsletter.slug);

        button(
            html!("View"),
            ButtonElement::A,
            &view_url,
            Some("button-primary")
        )
    } else {
        let send_url = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id);

        button(
            html!( "Publish" ),
            ButtonElement::A,
            &send_url,
            Some("button-primary")
        )
    };

    base(
        &page("Edit").htmx().alpine(ALPINE_ENTRY).islands(ISLANDS_ENTRY),
        html! {
        div.edit-view {
            @if newsletter.sent_at.is_some() {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header {
                div.left {
                    (button(
                        html!( "Back" ),
                        ButtonElement::A,
                        &back_url,
                        Some("button-secondary"))
                    )

                    span.save-status x-data="saveStatus()" x-text="status" { "Saved" }

                }

                div {
                    (view_or_preview_button)
                }
            }

            div data-island="Editor" data-props=(props) {}
        }
    })
}
