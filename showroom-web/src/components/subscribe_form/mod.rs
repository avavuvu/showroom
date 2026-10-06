use maud::{Markup, html};

use crate::components::{Kind, button, input};

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

                (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                (input("name").placeholder("name (optional)"))
                (button(html! { "Subscribe to "(publication_name) }).submit().primary())
            }
        }
    }
}
