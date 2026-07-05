use maud::{Markup, html};
use crate::views::{components::layout::header::header, context::PageContext};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    base(html! {
        (header(ctx))
        div.lander {
            div id="ascii-background" {}
            div.container {
                main {
                    p {
                        a.link href="/about" {
                            "Showroom"
                        }
                        " is currently in beta. "
                    }
                }
            }
        }
    })
}
