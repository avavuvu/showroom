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

pub fn confirmed(handle: &str) -> Markup {
    base(
        &ViewContext::metadata("Confirmed"),
        html! {
        div {
            h1 { "You're subscribed!" }
            p { "Your subscription to " (handle) " has been confirmed." }
        }
    })
}

pub fn unsubscribed(handle: &str) -> Markup {
    base(
        &ViewContext::metadata("Confirmed"),
        html! {
        div {
            h1 { "Sorry to see you go" }
            p { "You've been unsubscribed from " (handle) "." }
        }
    })
}
