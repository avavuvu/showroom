use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ViewContext, base};

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    let back_url = ctx.urls.app();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        let user = ctx.user.as_ref().expect("User must be authenticated");
        let view_url = format!("{}/{}", ctx.urls.user(&user.handle), newsletter.slug);

        button(
            html!("View"),
            ButtonElement::A,
            &view_url,
            Some("button-primary")
        )
    } else {
        let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);

        button(
            html!( "Publish" ),
            ButtonElement::A,
            &send_url,
            Some("button-primary")
        )
    };

    base(
        ViewContext::page("Edit").alpine().htmx().islands(),
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
