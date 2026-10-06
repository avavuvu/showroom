use boutique::html;
use maud::Markup;

use crate::components::{Button, Input, Kind};

pub fn subscribe_form(publication_url: &str, publication_name: &str) -> Markup {
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
