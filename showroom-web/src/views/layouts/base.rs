use maud::{DOCTYPE, Markup, html};

use crate::views::layouts::{ViewContext, view_context::Metadata};

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
                @if let Some(meta) = &context.metadata {
                    (seo_tags(&context.title, meta))
                }
            }
            body {
                (content)
                @if context.htmx {
                    script type="module" src="/assets/htmx.js" {}
                }
                @if context.alpine {
                    script type="module" src="/assets/alpine.js" {}
                }
                @for name in &context.scripts {
                    script type="module" src={ "/assets/" (name) ".js" } {}
                }
                @if context.islands {
                    script type="module" src="/assets/islands.js" {}
                }
            }
        }
    }
}

fn seo_tags(title: &str, meta: &Metadata) -> Markup {
    let twitter_card = if meta.image.is_some() { "summary_large_image" } else { "summary" };

    html! {
        meta name="description" content=(meta.description);
        meta property="og:type" content=(meta.og_type.as_str());
        meta property="og:title" content=(title);
        meta property="og:description" content=(meta.description);
        meta name="twitter:card" content=(twitter_card);
        meta name="twitter:title" content=(title);
        meta name="twitter:description" content=(meta.description);
        @if let Some(image) = &meta.image {
            meta property="og:image" content=(image);
            meta name="twitter:image" content=(image);
        }
        @if let Some(author) = &meta.author {
            meta name="author" content=(author);
        }
        @if let Some(url) = &meta.url {
            meta property="og:url" content=(url);
            link rel="canonical" href=(url);
        }
        @if let Some(time) = &meta.modified_time {
            meta property="article:modified_time" content=(time);
        }
    }
}
