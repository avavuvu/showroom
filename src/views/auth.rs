use maud::{Markup, html};
use crate::views::components::forms::{input::input, password_input::password_input};

use super::layouts::base;

pub fn login(error: Option<&str>) -> Markup {
    base(html! {
        div {
            div {
                h1 { "Sign in" }
                div id="login-error" {
                    @if let Some(err) = error {
                        p { (err) }
                    }
                }
                form
                    method="POST"
                    action="/login"
                    novalidate?[true]
                    hx-post="/login"
                    hx-trigger="validated"
                    hx-target="#login-error"
                    hx-swap="innerHTML"
                    x-data="form()"
                    x-on:submit="validate($event)"
                {
                    (input("email", "email", "email", "email", "you@example.com", true))
                    (password_input("password", "Password"))
                    button type="submit" { "Sign in" }
                }
                p {
                    "No account? "
                    a href="/signup" { "Sign up" }
                }
            }
        }
    })
}

pub fn signup(error: Option<&str>) -> Markup {
    base(html! {
        div {
            div {
                h1 { "Create account" }
                div id="signup-error" {
                    @if let Some(err) = error {
                        p { (err) }
                    }
                }
                form
                    method="POST"
                    action="/signup"
                    novalidate?[true]
                    hx-post="/signup"
                    hx-trigger="validated"
                    hx-target="#signup-error"
                    hx-swap="innerHTML"
                    x-data="form()"
                    x-on:submit="validate($event)"
                {
                    (input("email", "email", "email", "email", "you@example.com", true))
                    div class="inline-flex" {
                        span { "@" }
                        (input("handle", "handle", "text", "off", "yourhandle", true))
                    }
                    (password_input("password", "Password"))
                    button type="submit" { "Create account" }
                }
                p {
                    "Already have an account? "
                    a href="/login" { "Sign in" }
                }
            }
        }
    })
}
