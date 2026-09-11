use maud::{Markup, html};
use crate::views::{PageContext, components::forms::{handle_input::handle_input, input::input, password_input::password_input}, layouts::{ALPINE_ENTRY, page, shell}};

pub fn login(ctx: &PageContext) -> Markup {
    shell(
        page("Login").htmx().alpine(ALPINE_ENTRY),
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
                    (input("email", "email", "email", "email", "you@example.com", true))
                    (password_input("password", "Password"))
                    button.button-primary type="submit" { "Sign in" }
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
        page("Get started").htmx().alpine(ALPINE_ENTRY),
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
                    (input("email", "email", "email", "email", "you@example.com", true))
                    (handle_input())
                    (password_input("password", "Password"))
                    button.button-primary type="submit" { "Create account" }

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
                    (input("email", "email", "email", "email", "you@example.com", true))
                    button.button-primary type="submit" { "Send reset link" }
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
        page("Set new password").htmx().alpine(ALPINE_ENTRY),
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
                    (password_input("password", "New password"))
                    (password_input("password_confirm", "Confirm password"))
                    button.button-primary type="submit" { "Change password" }
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
