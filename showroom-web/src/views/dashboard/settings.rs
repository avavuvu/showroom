use maud::{Markup, html};
use crate::views::{components::ui::*, context::PageContext, layouts::{page, dashboard_shell}};

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
                (button(
                    html! { "New publication" },
                    ButtonElement::A,
                    &format!("{}/new", ctx.urls.app()),
                    Some("button-primary")
                ))
            }

            section.settings-section {
                h2 { "Change password" }
                p.hint { "A reset link will be sent to your email." }
                form.settings-form
                    hx-post={ (ctx.urls.app()) "/settings/change-password/request" }
                    hx-target="#change-password-result"
                    hx-swap="innerHTML" {
                    div id="change-password-result" {}
                    button.button.button-secondary type="submit" { "Send reset email" }
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
