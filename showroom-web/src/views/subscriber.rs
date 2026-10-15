use boutique::html;
use maud::Markup;
use crate::{components::Button, views::layouts::page};

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

pub fn confirm_subscription(publication_name: &str) -> Markup {
    base(
        &page("Confirm your subscription"),
        html! {
        form method="post" {
            h1 { "Confirm your subscription" }
            p { "Subscribe to " (publication_name) "?" }
            Button(submit = true) .primary { "Confirm" }
        }
    })
}

pub fn confirm_unsubscribe(publication_name: &str) -> Markup {
    base(
        &page("Unsubscribe"),
        html! {
        form method="post" {
            h1 { "Unsubscribe" }
            p { "Stop receiving emails from " (publication_name) "?" }
            Button(submit = true) .primary { "Unsubscribe" }
        }
    })
}

pub fn unsubscribed(publication_name: &str) -> Markup {
    base(
        &page("Confirmed"),
        html! {
        div {
            h1 { "Sorry to see you go" }
            p { "You've been unsubscribed from " (publication_name) "." }
        }
    })
}
