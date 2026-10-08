use boutique::html;
use maud::Markup;
use crate::{document::Document, models::newsletter::Model as Newsletter, renderer::html::render, components::{ArticleHeader, subscribe_form}, views::{context::PageContext, layouts::{Metadata, page, shell}}};

pub fn profile(ctx: &PageContext, newsletters: &[Newsletter]) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();

    let metadata = Metadata::article(
        publication.description.as_deref().unwrap_or(&publication.name),
        &publication.name,
        &publication_url);

    shell(
        page(&publication.name)
            .htmx()
            .seo(metadata),
        ctx,
        html! {
            div.user-view .article-layout .flow {
                h1 { (publication.name) }
                @if let Some(description) = &publication.description {
                    p.description { (description) }
                }
                (newsletter_list(newsletters, &publication_url))
            }
            (subscribe_form(&publication_url, &publication.name))
        }
    )
}

pub fn newsletter(newsletter: Newsletter, ctx: &PageContext) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();
    let content = render(&Document::from_stored(&newsletter.content));
    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let metadata = Metadata::article(
        newsletter.display_title(),
        &publication.name,
        &publication_url);

    shell(
        page(newsletter.display_title())
            .htmx()
            .seo(metadata),
        ctx,
        html! {
        main.article-layout .newsletter {
            article.prose .flow {
                ArticleHeader(
                    date = &date,
                    title = newsletter.display_title(),
                    subtitle = newsletter.subtitle.as_deref(),
                    name = &publication.name,
                    href = &publication_url,
                );
                (content)
            }
            div.subscribe {
                p {
                    "To recieve updates whenever "
                    a href=(publication_url) { (publication.name) }
                    " posts, consider subscribing."
                }
                (subscribe_form(&publication_url, &publication.name))
            }
        }
    })
}

fn newsletter_list(newsletters: &[Newsletter], publication_url: &str) -> Markup {
    html! {
        ul {
            @for newsletter in newsletters {
                li {
                    a href=(format!("{}/{}", publication_url, newsletter.slug)) { (newsletter.display_title()) }
                }
            }
        }
    }
}
