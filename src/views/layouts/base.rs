use maud::{DOCTYPE, Markup, html};

use crate::views::layouts::ViewContext;

pub fn base(context: &ViewContext, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" type="image/x-icon" href="/favicon.ico";
                title { (context.title) }
                link rel="stylesheet" href="/css/app.css";
                link rel="stylesheet" href="/css/prose.css";
            }
            body {
                (content)
                @if context.js {
                    script type="module" src="/assets/app.js" {}
                }
                @if context.islands {
                    script type="module" src="/assets/islands.js" {}
                }
            }
        }
    }
}
