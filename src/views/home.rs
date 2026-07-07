use maud::{Markup, html};
use crate::views::{components::layout::header::header, context::PageContext, layouts::{ViewContext, view_context::Metadata}};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    base(
        ViewContext::new("Showroom")
            .js("ascii")
            .seo(Metadata::website("A newsletter platform for the little guy")),
        html! {
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
