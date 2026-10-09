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
        header.header-full {
            @match banner {
                Some((banner, publication)) => {
                    a.logo-container href=(ctx.urls.publication(&publication.slug)) {
                        img.banner src=(banner) alt=(publication.name);
                    }
                }
                None => {
                    a.logo-container href=(ctx.urls.base()) {
                        img.logo src="/icons/logo-sm.webp" alt="";
                        Wordmark;
                    }
                }
            }

            div.auth {
                @if ctx.is_authenticated() {
                    Button(post = &format!("{}/logout", ctx.urls.base())) .ghost { "Log out" }
                    Button(href = &ctx.urls.app()) .ghost { "Dashboard" }
                } @else {
                    @if cfg!(debug_assertions) {
                        Button(href = &format!("{}/signup", ctx.urls.base())) .ghost { "Get started" }
                    }
                    Button(href = &format!("{}/login", ctx.urls.base())) .ghost { (login_text) }
                }
            }
        }
    }
}
