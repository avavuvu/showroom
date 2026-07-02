use maud::{Markup, PreEscaped, html};
use crate::{models::newsletter::Model as Newsletter, renderer::html::render, state::Urls, views::components::{forms::input::input, ui::{ButtonElement, button}}};
use super::layouts::base;

pub fn profile(handle: &str, is_authenticated: bool, urls: &Urls) -> Markup {
    let subscribe_to_url = &format!("{}/subscribe", urls.user(handle));

    base(html! {
        div {
            @if is_authenticated {
                form method="POST" action="/logout" {
                    button type="submit" { "Sign out" }
                }
            } @else {
                a href={ (urls.base()) "/login" } { "Sign in" }
            }

            h1 { (handle)"'s room" }

            div {
                div id="subscribe-error" {}
                form
                    method="POST"
                    action=(subscribe_to_url)
                    novalidate?[true]
                    hx-post=(subscribe_to_url)
                    hx-trigger="validated"
                    hx-target="#subscribe-error"
                    hx-swap="innerHTML"
                    x-data="form()"
                    x-on:submit="validate($event)" {

                    (input("email", "email", "email", "email", "you@example.com", true))
                    (input("name", "name", "text", "name", "name (optional)", true))
                    button type="submit" { "Subscribe to @"(handle) }
                }
            }


        }
    })
}

pub fn newsletter(newsletter: Newsletter) -> Markup {
    let html_string = render(&newsletter.content);

    base(html! {
        article.prose {
            h1 { (newsletter.title) }
            @if let Some(subtitle) = &newsletter.subtitle {
                p { (subtitle) }
            }

            div { (PreEscaped(html_string)) }
        }
    })
}
