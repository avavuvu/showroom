use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::shell};

pub fn about(ctx: &PageContext) -> Markup {
    shell(ctx, html! {
        main.article-layout .prose {
            div.flow  {
                h1 {
                    "About"
                }

                p {
                    "My name is "
                    a href="https://avavu.au" {
                        "Ava Dinh-Vu"
                    }
                    ", and for the past year I've been running a mailing list to keep my friends updated with my life and work and thoughts about the world."
                }

                p {
                    "Lots of my friends have wanted their own mailing lists, but that wasn't really feasible. I was running mine with a few jerry-rigged scripts that I had planned to replace with something better."
                }

                p {
                    "Showroom is that something better. It's a newsletter service for the little guy. "
                }

                hr;

                p {
                    "Showroom is currently in a very alpha stage. Only my personal newsletter is running on the platform. If you would like updates about Showroom, you can "
                    a href=(format!("{}", &ctx.urls.user("ava"))) {
                        "subscribe to my mailing list."
                    }
                }

            }
        }
    })
}
