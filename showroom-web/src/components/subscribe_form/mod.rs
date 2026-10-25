use boutique::{component, html};
use maud::Markup;

use crate::components::{Button, Input, Kind};

#[component]
pub fn subscribe_form(#[positional] publication_url: &str, #[positional] publication_name: &str) -> Markup {
    let subscribe_to_url = &format!("{}/subscribe", publication_url);

    html! {
        div.subscribe-form id="subscribe-form" {
            form
                method="POST"
                action=(subscribe_to_url)
                hx-post=(subscribe_to_url)
                hx-target="#subscribe-form"
                hx-swap="outerHTML" {

                Input("email", kind = Kind::Email) placeholder="you@example.com" required;
                Input("name") placeholder="name (optional)";
                Button(submit = true) .primary { "Subscribe to " (publication_name) }
            }
        }
    }
}
