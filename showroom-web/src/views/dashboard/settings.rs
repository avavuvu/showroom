use maud::{Markup, html};
use crate::views::{components::ui::*, context::PageContext, layouts::{ViewContext, base, dashboard_shell}};

pub fn index(ctx: &PageContext) -> Markup {
    let domain = ctx.urls.domain();

    dashboard_shell(
        ViewContext::page("Account").alpine().htmx().class("settings".into()),
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

pub fn change_password_form(token: &str) -> Markup {
    base(&ViewContext::page("Set New Password").alpine().htmx(), html! {
        h1 { "Set New Password" }
        form
            hx-post="/settings/change-password"
            hx-target="#result"
            hx-swap="outerHTML" {
            div id="result" {}
            input type="hidden" name="token" value=(token);
            label {
                "New password"
                input type="password" name="password" required minlength="8";
            }
            label {
                "Confirm password"
                input type="password" name="password_confirm" required;
            }
            button type="submit" { "Change Password" }
        }
    })
}

pub fn change_password_requested() -> Markup {
    html! { p { "Reset email sent. Check your inbox — the link expires in 1 hour." } }
}

pub fn change_password_error(message: &str) -> Markup {
    html! { p { (message) } }
}

pub fn change_password_success() -> Markup {
    base(&ViewContext::page("Password Changed"), html! {
        h1 { "Password changed" }
        p { "Your password has been updated." }
        a href="/settings" { "Back to account" }
    })
}
