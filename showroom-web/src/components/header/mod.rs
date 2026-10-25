use boutique::html;
use maud::Markup;
use crate::components::{Button, Wordmark};
use crate::views::context::PageContext;

pub fn header(ctx: &PageContext) -> Markup {
    let login_text = if cfg!(debug_assertions) {
        "Login"
    } else {
        "Admin"
    };

    let banner = ctx.publication.as_ref().and_then(|publication| {
        publication.banner_url(&ctx.urls).map(|banner| (banner, publication))
    });

    html! {
        div.header-space {}
        header.header-full.banner[banner.is_some()] {
            a.logo-container href=(ctx.urls.base()) {
                img.logo src="/icons/logo-sm.webp" alt="";
            }


            @match banner {
                Some((banner, publication)) => {
                    a.banner style="flex-grow: 0;" href=(ctx.urls.publication(&publication.slug)) {
                        img.banner src=(banner) alt=(publication.name);
                    }
                }
                None => {
                    a.banner href=(ctx.urls.base()) {
                        Wordmark;
                    }
                }
            }

            div.auth{
                @if ctx.is_authenticated() {
                    // Button(post = &format!("{}/logout", ctx.urls.base())) { "Log out" }
                    Button(href = &ctx.urls.app())  { "Dashboard" }
                } @else {
                    // @if cfg!(debug_assertions) {
                    //     Button(href = &format!("{}/signup", ctx.urls.base())) { "Get started" }
                    // }
                    Button(href = &format!("{}/login", ctx.urls.base())) { (login_text) }
                }
            }
        }
    }
}
