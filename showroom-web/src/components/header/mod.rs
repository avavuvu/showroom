use maud::{Markup, html};
use crate::components::button;
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
                    (button(html!{"Log out"}).post(format!("{}/logout", ctx.urls.base())))
                    (button(html!{"Dashboard"}).href(ctx.urls.app()))
                } @else {
                    @if cfg!(debug_assertions) {
                        (button(html!{"Get started"}).href(format!("{}/signup", ctx.urls.base())))
                    }
                    (button(html!{(login_text)}).href(format!("{}/login", ctx.urls.base())))
                }
            }
        }
    }
}
