use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::{ViewContext, base, dashboard_shell}};

pub fn index(ctx: &PageContext) -> Markup {
    dashboard_shell(
        ViewContext::page("Settings").alpine().htmx(),
        ctx,
        html! {
            h1 { "Settings" }

            section {
                h2 { "Change Password" }
                form
                    hx-post="/settings/change-password/request"
                    hx-target="#change-password-result"
                    hx-swap="innerHTML" {
                    div id="change-password-result" {}
                    p { "A reset link will be sent to your email." }
                    button type="submit" { "Send reset email" }
            }
        }
    })
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
        a href="/settings" { "Back to settings" }
    })
}
