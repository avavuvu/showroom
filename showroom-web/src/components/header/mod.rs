use boutique::html;
use maud::Markup;
use crate::components::Button;
use crate::views::context::PageContext;

pub fn header(ctx: &PageContext) -> Markup {
    let login_text = if cfg!(debug_assertions) {
        "Login"
    } else {
        "Admin"
    };

    html! {
        div.header-space {}
        header.header-full {
            a.logo-container href=(ctx.urls.base()) {
                img.logo src="/icons/logo-sm.webp" alt="";
                img.wordmark src="/icons/wordmark.svg" alt="Showroom";
            }

            div.auth {
                @if ctx.is_authenticated() {
                    Button(post = format!("{}/logout", ctx.urls.base())) .ghost { "Log out" }
                    Button(href = ctx.urls.app()) .ghost { "Dashboard" }
                } @else {
                    @if cfg!(debug_assertions) {
                        Button(href = format!("{}/signup", ctx.urls.base())) .ghost { "Get started" }
                    }
                    Button(href = format!("{}/login", ctx.urls.base())) .ghost { (login_text) }
                }
            }
        }
    }
}
