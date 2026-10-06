use maud::{Markup, html};
use crate::models::newsletter;
use crate::components::button;
use crate::views::context::PageContext;
use crate::views::layouts::{page, dashboard_shell};

pub fn index(ctx: &PageContext, newsletters: Vec<newsletter::Model>) -> Markup {
    let publication_url = ctx.publication_url();
    let dashboard_url = ctx.dashboard_url();
    let (published, drafts): (Vec<_>, Vec<_>) = newsletters.into_iter().partition(|n| n.sent_at.is_some());

    dashboard_shell(
        page(&ctx.publication().name).htmx().class("overview".into()),
        ctx,
        html! {
        div.dashboard-content {
            div.newsletters {
                section.newsletter-section.drafts {
                    header.section-header {
                        h2 { "Drafts" }
                        (button(html! { "New newsletter" }).post(format!("{}/newsletters", dashboard_url)).primary())
                    }
                    (newsletter_list(&drafts, &publication_url, &dashboard_url, "No drafts. Start a new newsletter."))
                }

                section.newsletter-section.published {
                    header.section-header {
                        h2 { "Published" }
                        @if !published.is_empty() {
                            p.section-count { (published.len()) " sent" }
                        }
                    }
                    (newsletter_list(&published, &publication_url, &dashboard_url, "Nothing published yet."))
                }
            }

            aside.comments {
                header.section-header {
                    h2 { "Recent comments" }
                }
                div.empty-state {
                    p { "No comments yet." }
                }
            }
        }
    })
}

fn newsletter_list(newsletters: &[newsletter::Model], publication_url: &str, dashboard_url: &str, empty_message: &str) -> Markup {
    html! {
        @if newsletters.is_empty() {
            div.empty-state {
                p { (empty_message) }
            }
        } @else {
            ul.newsletter-list {
                @for newsletter in newsletters {
                    (newsletter_row(newsletter, publication_url, dashboard_url))
                }
            }
        }
    }
}

fn newsletter_row(newsletter: &newsletter::Model, publication_url: &str, dashboard_url: &str) -> Markup {
    let view_url = format!("{}/{}", publication_url, newsletter.slug);
    let edit_url = format!("{}/edit/{}", dashboard_url, newsletter.id);

    let (primary_url, date) = match newsletter.sent_at {
        Some(sent_at) => (view_url.as_str(), format!("Sent {}", sent_at.format("%b %-d, %Y"))),
        None => (edit_url.as_str(), format!("Edited {}", newsletter.updated_at.format("%b %-d, %Y"))),
    };

    html! {
        li.newsletter-row id={ "newsletter-" (newsletter.id) } {
            div.newsletter-meta {
                h3 { a href=(primary_url) { (newsletter.title) } }
                @if let Some(subtitle) = &newsletter.subtitle {
                    p.subtitle { (subtitle) }
                }
                p.date { (date) }
            }

            div.newsletter-actions {
                @if newsletter.sent_at.is_some() {
                    a href=(view_url) { "View" }
                }
                a href=(edit_url) { "Edit" }
                (button(html! { "Delete" })
                    .danger()
                    .hx_delete(format!("{dashboard_url}/newsletters/{}", newsletter.id))
                    .hx_target(format!("#newsletter-{}", newsletter.id))
                    .hx_swap("delete")
                    .hx_confirm("Delete this newsletter?")
                    .hx_status_4xx("swap:none")
                    .hx_status_5xx("swap:none"))
            }
        }
    }
}
