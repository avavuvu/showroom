use boutique::html;
use maud::Markup;
use crate::{components::{AsciiBackground, FruitsBackground, header}, views::{context::PageContext, layouts::{Metadata, page}}};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    base(
        &page("Showroom")
            .seo(Metadata::website("A newsletter platform for the little guy")),
        html! {
        (header(ctx))
        div.lander {
            // AsciiBackground(
            //     webm = "/assets/flower-loop.webm",
            //     mp4 = "/assets/flower-loop.mp4",
            //     still = "/assets/flower-loop-ascii-landscape.svg",
            //     still_portrait = "/assets/flower-loop-ascii-portrait.svg",
            // );
            FruitsBackground;
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
