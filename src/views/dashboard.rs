use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ViewContext, newsletter_template};
use super::layouts::base;

pub fn index(ctx: &PageContext) -> Markup {
    let user = ctx.user.as_ref().expect("dashboard requires authentication");
    base(
        ViewContext::page("Dashboard").alpine().htmx(),
        html! {
        div {
            h1 { "Your dashboard" }
            p { "Welcome, " (user.handle) }
            form method="POST" action={ (ctx.urls.base()) "/logout" } {
                button type="submit" { "Sign out" }
            }
            div
                hx-get="/newsletters"
                hx-trigger="load"
                hx-swap="outerHTML" {
                "Loading..."
            }
            nav {
                (button(
                    html! { "Create a new newsletter" },
                    ButtonElement::Form,
                    &format!("{}/newsletters", ctx.urls.app()),
                    None
                ))
            }
        }
    })
}

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);
    let back_url = format!("{}/edit/{}", ctx.urls.app(), newsletter.id);

    let user = ctx.user.as_ref().expect("dashboard requires authentication");

    let user_url = ctx.urls.user(&user.handle);

    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let template = newsletter_template(
        &newsletter.title,
        newsletter.subtitle.as_deref(),
        &user.handle,
        &date,
        &user_url,
        &user_url,
        None,
        newsletter.rendered.as_ref().expect("")
    );

    // it shouldnt be possible for this to not be rendered,
    // because it gets rendered in the handler
    base(
        ViewContext::page(&newsletter.title),
        html! {
        div.preview-view {
            header {
                div.left {
                    (button(
                        html!( "Back" ),
                        ButtonElement::A,
                        &back_url,
                        Some("button-secondary"))
                    )
                }

                div {
                    (button(
                        html!( "Send" ),
                        ButtonElement::Form,
                        &send_url,
                        Some("button-primary"))
                    )
                }
            }

            article.flow {
                div.emails {
                    span {
                        "FROM:"
                    }
                    span {
                        (&ctx.urls.email(&user.handle))
                    }
                    span {
                        "TO:"
                    }
                    span {
                        "avadinhvu@gmail.com"
                    }
                    span {
                        "SUBJ:"
                    }
                    span {
                        (&newsletter.title)
                    }
                }
            }

            (template)

        }
    })
}

pub fn edit(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter.id }).to_string();
    let back_url = ctx.urls.app();
    let view_or_preview_button = if newsletter.sent_at.is_some() {
        let user = ctx.user.as_ref().expect("User must be authenticated");
        let view_url = format!("{}/{}", ctx.urls.user(&user.handle), newsletter.slug);

        button(
            html!("View"),
            ButtonElement::A,
            &view_url,
            Some("button-primary")
        )
    } else {
        let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);

        button(
            html!( "Publish" ),
            ButtonElement::A,
            &send_url,
            Some("button-primary")
        )
    };

    base(
        ViewContext::page("Edit").alpine().htmx().islands(),
        html! {
        div.edit-view {
            @if newsletter.sent_at.is_some() {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header {
                div.left {
                    (button(
                        html!( "Back" ),
                        ButtonElement::A,
                        &back_url,
                        Some("button-secondary"))
                    )

                    span.save-status x-data="saveStatus()" x-text="status" { "Saved" }

                }

                div {
                    (view_or_preview_button)
                }
            }

            div data-island="Editor" data-props=(props) {}
        }
    })
}

pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    html! {
        ul {
            @for newsletter in &newsletters {
                li {
                    h2 { (newsletter.title) }
                    a href={ (user_base) "/" (newsletter.slug) } { "View" }
                    a href={ "/edit/" (newsletter.id) } { "Edit" }
                }
            }
        }
    }
}
