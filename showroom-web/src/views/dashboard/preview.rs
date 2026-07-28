use maud::{Markup, html};
use crate::models::newsletter;
use crate::renderer::email::{ThemeVariables, render_email};
use crate::views::components::ui::*;
use crate::views::context::PageContext;
use crate::views::layouts::{ViewContext, newsletter_template, base};

pub fn preview(ctx: &PageContext, newsletter: &newsletter::Model) -> Markup {
    let send_url = format!("{}/send/{}", ctx.urls.app(), newsletter.id);
    let back_url = format!("{}/edit/{}", ctx.urls.app(), newsletter.id);

    let user = ctx.user.as_ref().expect("dashboard requires authentication");

    let user_url = ctx.urls.user(&user.handle);

    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let content = render_email(&newsletter.content, ThemeVariables::default());

    let template = newsletter_template(
        &newsletter.title,
        newsletter.subtitle.as_deref(),
        &user.handle,
        &date,
        &user_url,
        &user_url,
        None,
        content,
    );

    base(
        &ViewContext::page(&newsletter.title),
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
