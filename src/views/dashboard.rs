use maud::{Markup, html};
use crate::models::{newsletter, user};
use crate::views::components::ui::*;
use super::layouts::base;

pub fn index(user: &user::Model, base_url: &str) -> Markup {
    base(html! {
        div {
            h1 { "Your dashboard" }
            p { "Welcome, " (user.handle) }
            form method="POST" action={ (base_url) "/logout" } {
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
                    &format!("{}/newsletter", base_url)
                ))
            }
        }
    })
}

pub fn preview(app_url: &str, newsletter: &newsletter::Model) -> Markup {

    let send_url = format!("{}/send/{}", app_url, newsletter.id);

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

pub fn edit(app_url: &str, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();

    let send_url = format!("{}/send/{}", app_url, newsletter.id);

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
