use maud::{Markup, html};
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
