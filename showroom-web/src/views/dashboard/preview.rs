use maud::{Markup, html};
use crate::models::newsletter;
use crate::renderer::email::{ThemeVariables, render_email};
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{page, newsletter_template, base};

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let publication = ctx.publication();
    let send_url = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id);
    let back_url = format!("{}/edit/{}", ctx.dashboard_url(), newsletter.id);
    let publication_url = ctx.publication_url();

    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let content = render_email(&newsletter.content, ThemeVariables::default());

    let template = newsletter_template(
        &newsletter.title,
        newsletter.subtitle.as_deref(),
        &publication.name,
        &date,
        &publication_url,
        &publication_url,
        None,
        content,
    );

    base(
        &page(&newsletter.title),
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
                        (publication.name) " <" (ctx.urls.email(&publication.slug)) ">"
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
