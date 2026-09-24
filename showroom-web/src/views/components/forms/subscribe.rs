use maud::{Markup, html};

use boutique::components::{Button, Input};

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

                (Input::email("email").placeholder("you@example.com").required())
                (Input::text("name").placeholder("name (optional)"))
                (Button::submit(html! { "Subscribe to "(publication_name) }).primary())
            }
        }
    }
}
