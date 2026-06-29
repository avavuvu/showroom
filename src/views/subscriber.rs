use maud::{Markup, html};
use super::layouts::base;

pub fn subscribe_success() -> Markup {
    base(html! {
        div {
            h1 { "Check your email" }
            p { "We've sent you a confirmation link. Click it to complete your subscription." }
        }
    })
}

pub fn subscribe_exists() -> Markup {
    base(html! {
        div {
            h1 { "Already subscribed" }
            p { "This email is already subscribed. Check your inbox for the confirmation email if you haven't confirmed yet." }
        }
    })
}

pub fn confirmed(handle: &str) -> Markup {
    base(html! {
        div {
            h1 { "You're subscribed!" }
            p { "Your subscription to " (handle) " has been confirmed." }
        }
    })
}

pub fn unsubscribed(handle: &str) -> Markup {
    base(html! {
        div {
            h1 { "Sorry to see you go" }
            p { "You've been unsubscribed from " (handle) "." }
        }
    })
}
