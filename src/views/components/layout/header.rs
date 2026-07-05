use maud::{Markup, html};
use crate::views::{components::ui::*, context::PageContext};

pub fn header(ctx: &PageContext) -> Markup {
    html! {
        div.header-space {}
        header.header-full {
            a.logo-container href=(ctx.urls.base()) {
                img.logo src="/icons/logo-sm.webp" alt="";
                img.wordmark src="/icons/wordmark.svg" alt="Showroom";
            }
            div.auth {
                @if ctx.is_authenticated() {
                    (button(html!{"Log out"}, ButtonElement::Form, &format!("{}/logout", ctx.urls.base()), None))
                    (button(html!{"Dashboard"}, ButtonElement::A, &ctx.urls.app(), None))
                } @else {
                    (button(html!{"Get started"}, ButtonElement::A, &format!("{}/signup", ctx.urls.base()), None))
                    (button(html!{"Login"}, ButtonElement::A, &format!("{}/login", ctx.urls.base()), None))
                }
            }
        }
    }
}
