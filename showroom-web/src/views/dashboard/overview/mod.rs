use boutique::html;
use maud::Markup;
use crate::models::newsletter;
use crate::components::{Button, LocalTime};
use crate::services::comments::{Badge, Shown};
use crate::state::Urls;
use crate::views::context::PageContext;
use crate::views::layouts::{page, dashboard_shell};

pub fn index(ctx: &PageContext, newsletters: Vec<newsletter::Model>, recent: &[(Shown, newsletter::Model)]) -> Markup {
    let publication_url = ctx.publication_url();
    let dashboard_url = ctx.dashboard_url();
    let (published, drafts): (Vec<_>, Vec<_>) = newsletters.into_iter().partition(|n| n.published_at.is_some());

    dashboard_shell(
        page(&ctx.publication().name).htmx().class("overview"),
        ctx,
        html! {
        div.dashboard-content {
            div.newsletters {
                section.newsletter-section.drafts {
                    header.section-header {
                        h2 { "Drafts" }
                        Button(post = &format!("{}/newsletters", dashboard_url)) .primary { "New newsletter" }
                    }
                    (newsletter_list(&ctx.urls, &drafts, &publication_url, &dashboard_url, "No drafts. Start a new newsletter."))
                }

                section.newsletter-section.published {
                    header.section-header {
                        h2 { "Published" }
                        @if !published.is_empty() {
                            p.section-count { (published.len()) " sent" }
                        }
                    }
                    (newsletter_list(&ctx.urls, &published, &publication_url, &dashboard_url, "Nothing published yet."))
                }
            }

            aside.recent-comments {
                header.section-header {
                    h2 { "Recent comments" }
                }
                @if recent.is_empty() {
                    div.empty-state {
                        p { "No comments yet." }
                    }
                } @else {
                    ul.comment-list {
                        @for (comment, newsletter) in recent {
                            (comment_row(&ctx.urls, comment, newsletter, &publication_url, &dashboard_url))
                        }
                    }
                }
            }
        }
    })
}

fn comment_row(urls: &Urls, comment: &Shown, newsletter: &newsletter::Model, publication_url: &str, dashboard_url: &str) -> Markup {
    let comment_url = urls.visit(&format!("{}/{}#comment-{}", publication_url, newsletter.slug, comment.id));

    html! {
        li.comment-row id={ "dashboard-comment-" (comment.id) } {
            p.comment-meta {
                span.comment-name { (comment.name) }
                @match comment.badge {
                    Some(Badge::Author) => { " " span.comment-badge.author { "Author" } },
                    Some(Badge::Member) => { " " span.comment-badge { "Showroom" } },
                    None => {},
                }
                " on "
                a href=(comment_url) { (newsletter.display_title()) }
            }
            div.comment-body.comment-rich { (comment.content.html()) }
            div.comment-actions {
                LocalTime(&comment.created_at);
                Button
                    .danger
                    hx-delete={ (dashboard_url) "/comments/" (comment.id) }
                    hx-target={ "#dashboard-comment-" (comment.id) }
                    hx-swap="delete"
                    hx-confirm="Delete this comment?"
                    hx-status:4xx="swap:none"
                    hx-status:5xx="swap:none"
                {
                    "Delete"
                }
            }
        }
    }
}

fn newsletter_list(urls: &Urls, newsletters: &[newsletter::Model], publication_url: &str, dashboard_url: &str, empty_message: &str) -> Markup {
    html! {
        @if newsletters.is_empty() {
            div.empty-state {
                p { (empty_message) }
            }
        } @else {
            ul.newsletter-list {
                @for newsletter in newsletters {
                    (newsletter_row(urls, newsletter, publication_url, dashboard_url))
                }
            }
        }
    }
}

fn newsletter_row(urls: &Urls, newsletter: &newsletter::Model, publication_url: &str, dashboard_url: &str) -> Markup {
    let view_url = urls.visit(&format!("{}/{}", publication_url, newsletter.slug));
    let edit_url = format!("{}/edit/{}", dashboard_url, newsletter.id);
    let send_url = format!("{}/send/{}", dashboard_url, newsletter.id);

    let (primary_url, date) = match newsletter.published_at {
        Some(published_at) => (view_url.as_str(), format!("Sent {}", published_at.format("%b %-d, %Y"))),
        None => (edit_url.as_str(), format!("Edited {}", newsletter.updated_at.format("%b %-d, %Y"))),
    };

    html! {
        li.newsletter-row id={ "newsletter-" (newsletter.id) } {
            div.newsletter-meta {
                h3 { a href=(primary_url) { (newsletter.display_title()) } }
                @if let Some(subtitle) = &newsletter.subtitle {
                    p.subtitle { (subtitle) }
                }
                p.date { (date) }
            }

            div.newsletter-actions {
                @if newsletter.published_at.is_some() {
                    a href=(view_url) { "View" }
                }
                @if newsletter.send_started_at.is_some() {
                    a href=(send_url) { "Delivery" }
                }
                a href=(edit_url) { "Edit" }
                Button
                    .danger
                    hx-delete={ (dashboard_url) "/newsletters/" (newsletter.id) }
                    hx-target={ "#newsletter-" (newsletter.id) }
                    hx-swap="delete"
                    hx-confirm="Delete this newsletter?"
                    hx-status:4xx="swap:none"
                    hx-status:5xx="swap:none"
                {
                    "Delete"
                }
            }
        }
    }
}
