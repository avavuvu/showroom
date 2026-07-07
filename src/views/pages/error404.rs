use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::{ViewContext, base, shell}};

pub fn lander_404(ctx: &PageContext) -> Markup {
    shell(ViewContext::page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.urls.base()) { "Back to home" }
        }
    })
}

pub fn app_404(ctx: &PageContext) -> Markup {
    shell(ViewContext::page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.urls.app()) { "Back to dashboard" }
        }
    })
}

pub fn user_404(ctx: &PageContext) -> Markup {
    let owner = ctx.page_owner.as_ref().expect("user_404 requires page_owner");
    shell(ViewContext::page("404"), ctx, html! {
        main.article-layout .flow .prose {
            h1 { "404" }
            p { "Page not found." }
            a href=(ctx.urls.user(&owner.handle)) { "Back to profile" }
        }
    })
}
