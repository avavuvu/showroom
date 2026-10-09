use boutique::html;
use boutique::views::base;
use maud::{Markup, PreEscaped};

use crate::components::Button;
use crate::models::newsletter;
use crate::services::newsletter_send::Progress;
use crate::views::context::PageContext;
use crate::views::layouts::{page as head};

pub fn page(ctx: &PageContext, newsletter: &newsletter::Model, progress: &Progress) -> Markup {
    let view_url = format!("{}/{}", ctx.publication_url(), newsletter.slug);
    let back_url = format!("{}/edit/{}", ctx.dashboard_url(), newsletter.id);
    let title = format!("Sending – {}", newsletter.display_title());

    base(
        &head(&title)
            .htmx()
            .class("sending".into()),
        html! {
            style { (PreEscaped(ctx.publication().theme().css())) }
            div.preview-view {
                header {
                    div.left {
                        Button(href = &back_url) .secondary { "Back" }
                    }

                    div.right {
                        Button(href = &view_url) .primary { "View online" }
                    }
                }

                article.flow {
                    h1 { (title) }
                    (panel(ctx, newsletter, progress))
                }
            }
        }
    )
}

pub fn panel(ctx: &PageContext, newsletter: &newsletter::Model, progress: &Progress) -> Markup {
    let send_url = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id);
    let active = progress.active();

    let heading = match () {
        _ if active => "Sending…",
        _ if progress.total() == 0 => "Sent. There were no subscribers.",
        _ if progress.sent == 0 => "Sending stopped",
        _ if progress.failed + progress.unknown > 0 => "Sent, with problems",
        _ => "Sent",
    };

    html! {
        div #send-progress .send-progress
            hx-get=[active.then(|| format!("{send_url}/progress"))]
            hx-trigger=[active.then_some("every 2s")]
            hx-swap=[active.then_some("outerHTML")]
        {
            p.send-status { (heading) }

            @if progress.total() > 0 {
                progress value=(progress.done()) max=(progress.total()) {}
                p.send-counts {
                    (progress.sent) " of " (progress.total()) " sent"
                    @if progress.queued + progress.sending > 0 { " · " (progress.queued + progress.sending) " waiting" }
                    @if progress.failed > 0 { " · " (progress.failed) " failed" }
                    @if progress.unknown > 0 { " · " (progress.unknown) " unknown" }
                    @if progress.skipped > 0 { " · " (progress.skipped) " skipped" }
                }
            }

            @if !progress.recent.is_empty() {
                ul.send-recent {
                    @for delivery in &progress.recent {
                        li { (delivery.email) }
                    }
                }
            }

            @if !progress.problems.is_empty() {
                h3 { "Not sent" }
                ul.send-problems {
                    @for delivery in &progress.problems {
                        li {
                            span.email { (delivery.email) }
                            @if let Some(error) = &delivery.error {
                                span.error { (error) }
                            }
                        }
                    }
                }
            }

            @if !active {
                div.send-actions {
                    @if progress.failed > 0 {
                        Button(post = &format!("{send_url}/retry")) .secondary { "Try the failed emails again" }
                    }
                }
            }
        }
    }
}
