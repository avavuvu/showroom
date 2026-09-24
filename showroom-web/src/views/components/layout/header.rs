use maud::{Markup, html};
use boutique::components::Button;
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
                    (Button::post(html!{"Log out"}, format!("{}/logout", ctx.urls.base())))
                    (Button::link(html!{"Dashboard"}, ctx.urls.app()))
                } @else {
                    @if cfg!(debug_assertions) {
                        (Button::link(html!{"Get started"}, format!("{}/signup", ctx.urls.base())))
                    }
                    (Button::link(html!{(login_text)}, format!("{}/login", ctx.urls.base())))
                }
            }
        }
    }
}
