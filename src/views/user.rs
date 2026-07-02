use maud::{Markup, PreEscaped, html};
use crate::{models::newsletter::Model as Newsletter, renderer::html::render, views::{components::forms::subscribe::subscribe_form, context::PageContext, layouts::shell}};
use super::layouts::base;

pub fn profile(ctx: &PageContext) -> Markup {
    let owner = ctx.page_owner.as_ref().expect("user profile requires page_owner");
    base(html! {
        div {
            @if ctx.is_authenticated() {
                form method="POST" action="/logout" { button type="submit" { "Sign out" } }
            } @else {
                a href={ (ctx.urls.base()) "/login" } { "Sign in" }
            }
            h1 { (owner.handle)"'s room" }
            (subscribe_form(&ctx.urls.user(&owner.handle), &owner.handle))
        }
    })
}

pub fn newsletter(newsletter: Newsletter, ctx: &PageContext) -> Markup {
    let owner = ctx.page_owner.as_ref().expect("newsletter view requires page_owner");
    let html_string = render(&newsletter.content);
    let date = newsletter.created_at.format("%B %-d, %Y").to_string();
    let user_url = ctx.urls.user(&owner.handle);

    shell(ctx, html! {
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
