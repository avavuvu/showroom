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
            div id="ascii-background" {
                video id="ascii-video"
                    autoplay?[true]
                    muted?[true]
                    loop?[true]
                    playsinline?[true]
                    {
                    source src="/assets/flower-loop.webm" type="video/webm";
                    source src="/assets/flower-loop.mp4" type="video/mp4";
                }
            }
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
