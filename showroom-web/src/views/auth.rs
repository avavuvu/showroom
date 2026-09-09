use maud::{Markup, html};
use crate::views::{PageContext, components::forms::{handle_input::handle_input, input::input, password_input::password_input}, layouts::{ViewContext, shell}};

pub fn login(ctx: &PageContext) -> Markup {
    shell(
        ViewContext::new("Login").alpine().htmx(),
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
        ViewContext::new("Get started").alpine().htmx(),
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
