use maud::{Markup, html};
use boutique::components::Button;
use crate::views::{context::PageContext, layouts::{page, dashboard_shell}};

pub fn index(ctx: &PageContext) -> Markup {
    let domain = ctx.urls.domain();

    dashboard_shell(
        page("Account").htmx().class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                h2 { "Publications" }
                p.hint { "Each publication has its own address, subscribers and newsletters." }
                ul.settings-list {
                    @for publication in &ctx.publications {
                        li {
                            a href=(ctx.urls.dashboard(&publication.slug)) { (publication.name) }
                            span.address { (publication.slug) "." (domain) }
                            @if publication.is_default {
                                span.badge { "Your room" }
                            }
                        }
                    }
                }
                (Button::link(html! { "New publication" }, format!("{}/new", ctx.urls.app())).primary())
            }

            section.settings-section {
                h2 { "Change password" }
                p.hint { "A reset link will be sent to your email." }
                form.settings-form
                    hx-post={ (ctx.urls.app()) "/settings/change-password/request" }
                    hx-target="#change-password-result"
                    hx-swap="innerHTML" {
                    div id="change-password-result" {}
                    (Button::submit(html! { "Send reset email" }).secondary())
                }
            }
        }
    )
}

pub fn change_password_requested() -> Markup {
    html! { p { "Reset email sent. Check your inbox — the link expires in 1 hour." } }
}

pub fn change_password_error(message: &str) -> Markup {
    html! { p { (message) } }
}
