use maud::{Markup, html};
use crate::{components::header, views::{context::PageContext, layouts::{page, Metadata}}};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    base(
        &page("Showroom")
            .entry("ascii")
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
