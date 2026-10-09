use axum::http::StatusCode;
use boutique::html;
use maud::Markup;
use crate::views::{context::PageContext, layouts::{page, shell}};

pub fn error_page(ctx: &PageContext, status: StatusCode, message: Option<Markup>) -> Markup {
    let fallback = if status.is_server_error() {
        "Something went wrong. Please try again."
    } else {
        status.canonical_reason().unwrap_or("The request could not be completed.")
    };

    let (href, label) = if ctx.user.is_some() {
        (ctx.urls.app(), "Back to dashboard")
    } else {
        (ctx.urls.base(), "Back to home")
    };

    shell(page(status.as_u16().to_string()), ctx, html! {
        main.article-layout .flow .prose {
            h1 { (status.as_u16()) }
            @match message {
                Some(message) => (message),
                None => p { (fallback) },
            }
            a href=(href) { (label) }
        }
    })
}
