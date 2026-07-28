use maud::{Markup, html};
use crate::views::layouts::ViewContext;

use super::layouts::base;

pub fn subscribe_success() -> Markup {
    html! {
        div.subscribe-form id="subscribe-form" {
            h2 { "Please check your email" }
            p { "We've sent you a confirmation link. Click it to complete your subscription." }
        }
    }
}

pub fn subscribe_exists() -> Markup {
    html! {
        div.subscribe-form id="subscribe-form" {
            h2 { "Already subscribed" }
            p { "This email is already subscribed to this newsletter." }
        }
    }
}

pub fn subscribe_error(message: &str) -> Markup {
    html! {
        div id="subscribe-form" {
            p { (message) }
        }
    }
}

pub fn unsubscribed(handle: &str) -> Markup {
    base(
        &ViewContext::page("Confirmed"),
        html! {
        div {
            h1 { "Sorry to see you go" }
            p { "You've been unsubscribed from " (handle) "." }
        }
    })
}
