use maud::{Markup, html};
use crate::models::newsletter;
use boutique::components::Button;
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    let back_url = ctx.dashboard_url();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        Button::link(html!("View"), format!("{}/{}", ctx.publication_url(), newsletter.slug)).primary()
    } else {
        Button::link(html!("Publish"), format!("{}/send/{}", ctx.dashboard_url(), newsletter.id)).primary()
    };

    base(
        &page("Edit").htmx().alpine().islands(),
        html! {
        div.edit-view {
            @if newsletter.sent_at.is_some() {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header {
                div.left {
                    (Button::link(html!("Back"), back_url).secondary())

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
