use boutique::html;
use maud::Markup;
use crate::{document::Document, models::{newsletter::Model as Newsletter, publication::{self, Picture}}, renderer::html::render, components::{ArticleHeader, subscribe_form}, state::Urls, views::{context::PageContext, layouts::{Metadata, page, shell}}};

fn with_share_image(metadata: Metadata, publication: &publication::Model, urls: &Urls) -> Metadata {
    match (publication.banner_url(urls), publication.picture()) {
        (Some(banner), _) => metadata.with_image(&banner),
        (None, Picture::Upload(_)) => metadata.with_image(&publication.picture_url(urls)),
        (None, Picture::Default(_)) => metadata,
    }
}

pub fn profile(ctx: &PageContext, newsletters: &[Newsletter]) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();

    let metadata = with_share_image(
        Metadata::article(publication.description.as_deref().unwrap_or(&publication.name), &publication.name, &publication_url),
        publication,
        &ctx.urls,
    );

    shell(
        page(&publication.name)
            .htmx()
            .seo(metadata),
        ctx,
        html! {
            div.user-view .article-layout .flow {
                img.publication-picture src=(publication.picture_url(&ctx.urls)) alt="";
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

    let metadata = with_share_image(
        Metadata::article(newsletter.display_title(), &publication.name, &publication_url),
        publication,
        &ctx.urls,
    );

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
                    picture = &publication.picture_url(&ctx.urls),
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
