use boutique::html;
use maud::Markup;
use crate::{components::Button, views::{PageContext, layouts::notice_page}};

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

pub fn confirm_subscription(ctx: &PageContext) -> Markup {
    notice_page(ctx, "Confirm your subscription", html! {
        p { "Subscribe to " strong { (ctx.publication().name) } "?" }
        form method="post" {
            Button(submit = true) .primary { "Confirm" }
        }
    })
}

pub fn confirm_unsubscribe(ctx: &PageContext) -> Markup {
    notice_page(ctx, "Unsubscribe", html! {
        p { "Stop receiving emails from " strong { (ctx.publication().name) } "?" }
        form method="post" {
            Button(submit = true) .primary { "Unsubscribe" }
        }
    })
}

pub fn unsubscribed(ctx: &PageContext) -> Markup {
    notice_page(ctx, "Sorry to see you go", html! {
        p { "You've been unsubscribed from " (ctx.publication().name) "." }
        a.link href=(ctx.publication_url()) { "Back to " (ctx.publication().name) }
    })
}
