use maud::{Markup, html};

use crate::views::components::forms::input::input;

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

                (input("email", "email", "email", "email", "you@example.com", true))
                (input("name", "name", "text", "name", "name (optional)", false))
                button.button-primary type="submit" { "Subscribe to "(publication_name) }
            }
        }
    }
}
