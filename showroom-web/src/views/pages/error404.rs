use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::{page, shell}};

pub fn lander_404(ctx: &PageContext) -> Markup {
    shell(page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.urls.base()) { "Back to home" }
        }
    })
}

pub fn app_404(ctx: &PageContext) -> Markup {
    shell(page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.urls.app()) { "Back to dashboard" }
        }
    })
}

pub fn publication_404(ctx: &PageContext) -> Markup {
    shell(page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.publication_url()) { "Back to " (ctx.publication().name) }
        }
    })
}
