use boutique::html;
use maud::Markup;
use crate::{components::{header, AsciiBackground}, views::{context::PageContext, layouts::{page, Metadata}}};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    base(
        &page("Showroom")
            .seo(Metadata::website("A newsletter platform for the little guy")),
        html! {
        (header(ctx))
        div.lander {
            AsciiBackground(webm = "/assets/flower-loop.webm", mp4 = "/assets/flower-loop.mp4");
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
