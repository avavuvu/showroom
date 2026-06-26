use maud::{Markup, html};
use crate::models::{newsletter, user};
use crate::renderer::html::{render_email, ProseVars};
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

pub fn preview(newsletter: &newsletter::Model, vars: ProseVars) -> Markup {
    let rendered = render_email(&newsletter.content, vars);
    base(html! {
        (maud::PreEscaped(rendered))
    })
}

pub fn edit(newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    base(html! {
        div data-island="Editor" data-props=(props) {}
    })
}


pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    html! {
        ul {
            @for newsletter in &newsletters {
                li {
                    h2 { (newsletter.title) }
                    a href={ (user_base) "/" (newsletter.slug) } { "View" }
                    a href={ "/edit/" (newsletter.slug) } { "Edit" }
                }
            }
        }
    }
}
