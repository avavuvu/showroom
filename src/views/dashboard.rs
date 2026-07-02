use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    let user = ctx.user.as_ref().expect("dashboard requires authentication");
    base(html! {
        div {
            h1 { "Your dashboard" }
            p { "Welcome, " (user.handle) }
            form method="POST" action={ (ctx.urls.base()) "/logout" } {
                button type="submit" { "Sign out" }
            }
            div
                hx-get="/newsletters"
                hx-trigger="load"
                hx-swap="outerHTML" {
                "Loading..."
            }
            nav {
                (button(
                    html! { "Create a new newsletter" },
                    ButtonElement::Form,
                    &format!("{}/newsletters", ctx.urls.app())
                ))
            }
        }
    })
}

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);

    // it shouldnt be possible for this to not be rendered,
    // because it gets rendered in the handler
    base(html! {
        (maud::PreEscaped(newsletter.rendered.as_deref().unwrap_or_default()))

        aside {
            (button(
                html!( "Send" ),
                ButtonElement::Form,
                &send_url)
            )
        }
    })
}

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);

    base(html! {
        div data-island="Editor" data-props=(props) {}

        aside {
            (button(
                html!( "Send" ),
                ButtonElement::A,
                &send_url)
            )
        }
    })
}

pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    html! {
        ul {
            @for newsletter in &newsletters {
                li {
                    h2 { (newsletter.title) }
                    a href={ (user_base) "/" (newsletter.slug) } { "View" }
                    a href={ "/edit/" (newsletter.id) } { "Edit" }
                }
            }
        }
    }
}
