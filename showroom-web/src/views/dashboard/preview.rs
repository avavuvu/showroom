use boutique::html;
use maud::{Markup, PreEscaped};
use crate::document::Document;
use crate::models::newsletter;
use crate::renderer::email::render_email;
use crate::components::Button;
use crate::views::context::PageContext;
use crate::views::layouts::{page, newsletter_template, base};

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let publication = ctx.publication();
    let send_url = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id);
    let back_url = format!("{}/edit/{}", ctx.dashboard_url(), newsletter.id);
    let publication_url = ctx.publication_url();

    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let theme = publication.theme().email();
    let content = render_email(&Document::from_stored(&newsletter.content), theme);
    let has_title = !newsletter.title.trim().is_empty();
    let recipient = ctx.user.as_ref().map(|user| user.email.as_str()).unwrap_or("");

    let template = newsletter_template(
        newsletter.display_title(),
        newsletter.subtitle.as_deref(),
        &publication.name,
        &date,
        &publication_url,
        &publication_url,
        theme,
        content,
    );

    base(
        &page(newsletter.display_title()),
        html! {
        style { (PreEscaped(publication.theme().css())) }
        div.preview-view {
            header {
                div.left {
                    Button(href = back_url) .secondary { "Back" }
                }

                div.right {
                    @if !has_title {
                        p.send-warning { "Add a title before you send." }
                    }
                    Button(post = send_url, disabled = !has_title) .primary { "Send" }
                }
            }

            article.flow {
                div.emails {
                    span {
                        "FROM:"
                    }
                    span {
                        (publication.name) " <" (ctx.urls.email(&publication.slug)) ">"
                    }
                    span {
                        "TO:"
                    }
                    span {
                        (recipient)
                    }
                    span {
                        "SUBJ:"
                    }
                    span {
                        (newsletter.display_title())
                    }
                }
            }

            (template)

        }
    })
}
