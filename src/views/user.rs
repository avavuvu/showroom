use maud::{Markup, PreEscaped, html};
use crate::models::newsletter::Model as Newsletter;
use super::layouts::base;

pub fn profile(username: &str, is_authenticated: bool, base_url: &str) -> Markup {
    base(html! {
        div {
            h1 { (username) }
            @if is_authenticated {
                form method="POST" action="/logout" {
                    button type="submit" { "Sign out" }
                }
            } @else {
                a href={ (base_url) "/login" } { "Sign in" }
            }
        }
    })
}

pub fn newsletter(newsletter: Newsletter) -> Markup {
    base(html! {
        article {
            h1 { (newsletter.title) }
            @if let Some(subtitle) = &newsletter.subtitle {
                p { (subtitle) }
            }
            @if let Some(rendered) = &newsletter.rendered_html {
                div { (PreEscaped(rendered)) }
            } @else {
                p { "This newsletter hasn't been rendered yet." }
            }
        }
    })
}
