use maud::{Markup, html};
use crate::components::{Kind, button, input};
use crate::views::{PageContext, layouts::{page, shell}};

pub fn login(ctx: &PageContext) -> Markup {
    shell(
        page("Login").htmx(),
        ctx, html! {
        div.article-layout {
            div.auth-form {
                h1 { "Log in" }
                p.error id="login-error" {}
                form.flow
                    method="POST"
                    action="/login"
                    hx-post="/login"
                    hx-target="#login-error"
                    hx-swap="innerHTML"
                {
                    (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                    (input("password").kind(Kind::Password).label("Password").required())
                    (button(html! { "Sign in" }).submit().primary())
                    p {
                        a.link href="/forgot-password" { "Forgot your password?" }
                    }
                    @if cfg!(debug_assertions) {
                        p {
                            "No account? "
                            a.link href="/signup" { "Sign up" }
                        }
                    }
                }
            }
        }
    })
}

pub fn signup(ctx: &PageContext) -> Markup {
    shell(
        page("Get started").htmx(),
        ctx,
        html! {
        div.article-layout {
            div.auth-form {
                h1 { "Create account" }
                div id="signup-error" {}
                form.flow
                    method="POST"
                    action="/signup"
                    hx-post="/signup"
                    hx-target="#signup-error"
                    hx-swap="innerHTML"
                {
                    (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                    (input("handle").prefix("@").placeholder("yourhandle").autocomplete("username").required())
                    (input("password").kind(Kind::Password).label("Password").autocomplete("new-password").required())
                    (button(html! { "Create account" }).submit().primary())

                    p {
                        "Already have an account? "
                        a.link href="/login" { "Sign in" }
                    }
                }
            }
        }
    })
}

pub fn forgot_password(ctx: &PageContext) -> Markup {
    shell(
        page("Forgot password").htmx(),
        ctx,
        html! {
        div.article-layout {
            div.auth-form id="forgot-password" {
                h1 { "Forgot your password?" }
                p { "Enter your email and we will send you a link to set a new one." }
                p.error id="forgot-error" {}
                form.flow
                    method="POST"
                    action="/forgot-password"
                    hx-post="/forgot-password"
                    hx-target="#forgot-password"
                    hx-swap="outerHTML"
                {
                    (input("email").kind(Kind::Email).placeholder("you@example.com").required())
                    (button(html! { "Send reset link" }).submit().primary())
                    p {
                        a.link href="/login" { "Back to login" }
                    }
                }
            }
        }
    })
}

pub fn forgot_password_sent() -> Markup {
    html! {
        div.auth-form id="forgot-password" {
            h1 { "Check your email" }
            p { "If an account exists for that address, a reset link is on its way. It expires in 1 hour." }
        }
    }
}

pub fn reset_password(ctx: &PageContext, token: &str) -> Markup {
    shell(
        page("Set new password").htmx(),
        ctx,
        html! {
        div.article-layout {
            div.auth-form {
                h1 { "Set a new password" }
                p.error id="reset-error" {}
                form.flow
                    method="POST"
                    action="/reset-password"
                    hx-post="/reset-password"
                    hx-target="#reset-error"
                    hx-swap="innerHTML"
                {
                    input type="hidden" name="token" value=(token);
                    (input("password").kind(Kind::Password).label("New password").autocomplete("new-password").required())
                    (input("password_confirm").kind(Kind::Password).label("Confirm password").autocomplete("new-password").required())
                    (button(html! { "Change password" }).submit().primary())
                }
            }
        }
    })
}

pub fn reset_password_invalid(ctx: &PageContext) -> Markup {
    shell(
        page("Link expired"),
        ctx,
        html! {
        div.article-layout {
            div.auth-form {
                h1 { "This link is no longer valid" }
                p { "It may have expired or already been used." }
                p {
                    a.link href="/forgot-password" { "Request a new link" }
                }
            }
        }
    })
}
