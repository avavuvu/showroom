use maud::{Markup, html};

use crate::views::components::forms::input::input;

pub fn subscribe_form(user_url: &str, handle: &str) -> Markup {
    let subscribe_to_url = &format!("{}/subscribe", user_url);

    html! {
        div.subscribe-form id="subscribe-form" {
            form
                method="POST"
                action=(subscribe_to_url)
                novalidate?[true]
                hx-post=(subscribe_to_url)
                hx-trigger="validated"
                hx-target="#subscribe-form"
                hx-swap="outerHTML"
                x-data="form()"
                x-on:submit="validate($event)" {

                (input("email", "email", "email", "email", "you@example.com", true))
                (input("name", "name", "text", "name", "name (optional)", false))
                button.button-primary type="submit" { "Subscribe to @"(handle) }
            }
        }
    }
}
