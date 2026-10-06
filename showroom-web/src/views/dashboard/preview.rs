use maud::{Markup, html};
use crate::models::newsletter;
use crate::renderer::email::render_email;
use crate::components::button;
use crate::views::context::PageContext;
use crate::views::layouts::{page, newsletter_template, base};

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let publication = ctx.publication();
    let send_url = format!("{}/send/{}", ctx.dashboard_url(), newsletter.id);
    let back_url = format!("{}/edit/{}", ctx.dashboard_url(), newsletter.id);
    let publication_url = ctx.publication_url();

    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let theme = publication.theme().email();
    let content = render_email(&newsletter.content, theme);

    let template = newsletter_template(
        &newsletter.title,
        newsletter.subtitle.as_deref(),
        &publication.name,
        &date,
        &publication_url,
        &publication_url,
        theme,
        content,
    );

    base(
        &page(&newsletter.title),
        html! {
        div.preview-view {
            header {
                div.left {
                    (button(html!("Back")).href(back_url).secondary())
                }

                div {
                    (button(html!("Send")).post(send_url).primary())
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
