use maud::{Markup, PreEscaped, html};
use crate::{models::newsletter::{self, Model as Newsletter}, renderer::html::render, views::{components::forms::subscribe::subscribe_form, context::PageContext, layouts::{ViewContext, shell}}};

pub fn profile(ctx: &PageContext) -> Markup {
    let owner = ctx.page_owner.as_ref().expect("user profile requires page_owner");
    shell(
        &ViewContext {
            title: format!("{} – Showroom", &owner.handle),
            islands: false,
            js: true,
        },
        ctx,
        html! {
            div.user-view .article-layout .flow {
                h1 { (owner.handle)"'s room" }

                div
                    hx-get="/newsletters"
                    hx-trigger="load"
                    hx-swap="outerHTML" {
                    "Loading..."
                }

            }
            (subscribe_form(&ctx.urls.user(&owner.handle), &owner.handle))
        }
    )
}

pub fn newsletter(newsletter: Newsletter, ctx: &PageContext) -> Markup {
    let owner = ctx.page_owner.as_ref().expect("newsletter view requires page_owner");
    let html_string = render(&newsletter.content);
    let date = newsletter.created_at.format("%B %-d, %Y").to_string();
    let user_url = ctx.urls.user(&owner.handle);

    shell(
        &ViewContext {
            title: format!("{} – Showroom", &owner.handle),
            islands: false,
            js: true,
        },
        ctx,
        html! {
        main.article-layout .newsletter {
            article.prose .flow {
                div.info {
                    p.date { (date) }
                    h1 { (newsletter.title) }
                    @if let Some(subtitle) = &newsletter.subtitle {
                        p.subtitle { (subtitle) }
                    }
                    p.handle {
                        a href=(user_url) { "@"(owner.handle) }
                    }
                }
                (PreEscaped(html_string))
            }
            div.subscribe {
                p {
                    "To recieve updates whenever "
                    a href=(user_url) { (owner.handle) }
                    " posts, consider subscribing."
                }
                (subscribe_form(&user_url, &owner.handle))
            }
        }
    })
}

pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    html! {
        ul {
            @for newsletter in &newsletters {
                li {
                    a href=(format!("{}/{}", user_base, newsletter.slug)) { (newsletter.title) }
                }
            }
        }
    }
}
