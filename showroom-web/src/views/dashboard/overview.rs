use maud::{Markup, html};
use crate::models::newsletter;
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ViewContext, dashboard_shell};

pub fn index(ctx: &PageContext) -> Markup {
    let user = ctx.user.as_ref().expect("dashboard requires authentication");

    dashboard_shell(
        ViewContext::page("Dashboard").alpine().htmx(),
        ctx,
        html! {
        div.dashboard-page {
                // (button(
                //     html! { "New Newsletter" },
                //     ButtonElement::Form,
                //     &format!("{}/newsletters", ctx.urls.app()),
                //     Some("button-primary")
                // ))


            div.dashboard-content {
                section.published {
                    h2 { "Published" }
                    div
                        hx-get="/newsletters"
                        hx-trigger="load"
                        hx-swap="outerHTML" {
                        "Loading..."
                    }
                }

                aside.comments {
                    h2 { "Recent Comments" }
                    div.empty-state {
                        p { "No comments yet." }
                    }
                }
            }
        }
    })
}

pub fn newsletters(newsletters: Vec<newsletter::Model>, user_base: &str) -> Markup {
    let (sent, draft): (Vec<_>, Vec<_>) = newsletters.into_iter().partition(|n| n.sent_at.is_some());

    html! {
        div id="newsletters" {
            @if sent.is_empty() {
                div.empty-state {
                    p { "No published newsletters yet." }
                }
            } @else {
                ul.newsletter-list {
                    @for newsletter in &sent {
                        (newsletter_row(newsletter, user_base))
                    }
                }
            }

            @if !draft.is_empty() {
                div.drafts-teaser {
                    div {
                        p.drafts-count {
                            (draft.len()) " draft" (if draft.len() == 1 { "" } else { "s" }) " in progress"
                        }
                    }
                    ul.newsletter-list.drafts-list {
                        @for newsletter in &draft {
                            (newsletter_row(newsletter, user_base))
                        }
                    }
                }
            }
        }
    }
}

fn newsletter_row(newsletter: &newsletter::Model, user_base: &str) -> Markup {
    html! {
        li id={ "newsletter-" (newsletter.id) } {
            div {
                h3 {
                    a href={ (user_base) "/" (newsletter.slug) } {
                        (newsletter.title)
                    }
                }
                @if let Some(subtitle) = &newsletter.subtitle {
                    p.subtitle {
                        (subtitle)
                    }
                }
            }

            div.newsletter-actions {
                @if newsletter.sent_at.is_some() {
                    a href={ (user_base) "/" (newsletter.slug) } { "View" }
                }
                a href={ "/edit/" (newsletter.id) } { "Edit" }
                button
                    hx-delete={ "/newsletters/" (newsletter.id) }
                    hx-target={ "#newsletter-" (newsletter.id) }
                    hx-swap="outerHTML"
                    hx-confirm="Delete this newsletter?" { "Delete" }
            }
        }
    }
}
