use maud::{Markup, html};
use crate::views::{PageContext, components::forms::{handle_input::handle_input, input::input, password_input::password_input}, layouts::{ViewContext, shell}};

pub fn login(ctx: &PageContext) -> Markup {
    shell(
        &ViewContext {
            title: "Login".into(),
            islands: false,
            js: true,
        },
        ctx, html! {
        div.article-layout {
            div.auth-form {
                h1 { "Log in" }
                p.error id="login-error" {}
                form.flow
                    method="POST"
                    action="/login"
                    novalidate?[true]
                    hx-post="/login"
                    hx-trigger="validated"
                    hx-target="#login-error"
                    hx-swap="innerHTML"
                    x-data="form()"
                    x-on:submit="validate($event)"
                {
                    (input("email", "email", "email", "email", "you@example.com", true))
                    (password_input("password", "Password"))
                    button.button-primary type="submit" { "Sign in" }
                    p {
                        "No account? "
                        a.link href="/signup" { "Sign up" }
                    }
                }
            }
        }
    })
}

pub fn signup(ctx: &PageContext) -> Markup {
    shell(
        &ViewContext {
            title: "Get started".into(),
            islands: false,
            js: true,
        },
        ctx,
        html! {
        div.article-layout {
            div.auth-form {
                h1 { "Create account" }
                div id="signup-error" {}
                form.flow
                    method="POST"
                    action="/signup"
                    novalidate?[true]
                    hx-post="/signup"
                    hx-trigger="validated"
                    hx-target="#signup-error"
                    hx-swap="innerHTML"
                    x-data="form()"
                    x-on:submit="validate($event)"
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
