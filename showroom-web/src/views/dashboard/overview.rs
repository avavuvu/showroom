use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ViewContext, base};

pub fn index(ctx: &PageContext) -> Markup {
    let user = ctx.user.as_ref().expect("dashboard requires authentication");



    let rows = 20;
    let cols = 24;
    base(
        ViewContext::page("Dashboard").alpine().htmx(),
        html! {
        div.dashboard {
            div {
                nav {
                    a.logo  {
                        "Logo"
                    }
                    a.posts href="/posts" {
                        "Posts"
                    }
                    a.subscribers href="/subscribers" {
                        "Subscribers"
                    }
                    a.settings href="/settings" {
                        "Settings"
                    }
                }
            }

            main {

            }

            // h1 { "Your dashboard" }
            // p { "Welcome, " (user.handle) }
            // nav {
            //     form method="POST" action={ (ctx.urls.base()) "/logout" } {
            //         button type="submit" { "Sign out" }
            //     }
            //     ul {
            //         li {
            //             a href="/settings" {
            //                 "Settings"
            //             }
            //         }
            //         li {
            //             a href="/subscribers" {
            //                 "Subscribers"
            //             }
            //         }
            //     }
            // }
            // div
            //     hx-get="/newsletters"
            //     hx-trigger="load"
            //     hx-swap="outerHTML" {
            //     "Loading..."
            // }
            // nav {
            //     (button(
            //         html! { "Create a new newsletter" },
            //         ButtonElement::Form,
            //         &format!("{}/newsletters", ctx.urls.app()),
            //         None
            //     ))
            // }
        }
    })
}

pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    html! {
        ul {
            @for newsletter in &newsletters {
                li id={ "newsletter-" (newsletter.id) } {
                    h2 { (newsletter.title) }
                    @if newsletter.sent_at.is_some() {
                        a href={ (user_base) "/" (newsletter.slug) } { "View" }
                    }
                    a href={ "/edit/" (newsletter.id) } { "Edit" }
                    button
                        hx-delete={ "/newsletters/" (newsletter.id) }
                        hx-target={ "#newsletter-" (newsletter.id) }
                        hx-swap="outerHTML"
                        hx-confirm="Delete this newsletter?" { "Delete" }
                }
            }
        }
    }
}
