use boutique::prelude::*;
use crate::{components::{ArticleHeader, SubscribeForm}, document::Document, images::{ImageSize, centred_card, picture_card, share_card}, models::{newsletter::Model as Newsletter, publication::{self, Picture}}, renderer::html::render, state::Urls, views::{context::PageContext, layouts::{Metadata, page, shell}}};
pub mod profile;

pub const DESCRIPTION_LENGTH: usize = 155;

pub fn newsletter_url(publication_url: &str, newsletter: &Newsletter) -> String {
    format!("{publication_url}/{}", newsletter.slug)
}

pub fn newsletter_description(newsletter: &Newsletter, document: &Document) -> String {
    newsletter
        .subtitle
        .as_deref()
        .map(str::trim)
        .filter(|subtitle| !subtitle.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| document.excerpt(DESCRIPTION_LENGTH))
}

pub fn publication_description(publication: &publication::Model) -> String {
    publication
        .description
        .as_deref()
        .map(str::trim)
        .filter(|description| !description.is_empty())
        .map_or_else(|| format!("Newsletters from {}", publication.name), str::to_string)
}

pub fn titled(title: &str, publication: &publication::Model) -> String {
    format!("{title} \u{b7} {}", publication.name)
}

fn newsletter_image(document: &Document, newsletter: &Newsletter, publication: &publication::Model, urls: &Urls) -> String {
    match document.first_image_id() {
        Some(public_id) => urls.image(public_id, ImageSize::Og),
        None => share_card(&urls.cloudinary_cloud, newsletter.display_title(), &publication.name, publication.theme()),
    }
}

pub fn publication_image(publication: &publication::Model, urls: &Urls) -> String {
    match publication.picture() {
        Picture::Upload(public_id) => picture_card(&urls.cloudinary_cloud, public_id, publication.theme()),
        Picture::Default(_) => centred_card(&urls.cloudinary_cloud, &publication.name, publication.theme()),
    }
}

pub fn json_ld(value: serde_json::Value) -> Markup {
    html! {
        script type="application/ld+json" { (PreEscaped(value.to_string().replace("</", "<\\/"))) }
    }
}

pub fn publication_entity(publication: &publication::Model, urls: &Urls) -> serde_json::Value {
    serde_json::json!({
        "@type": "Organization",
        "name": publication.name,
        "url": format!("{}/", urls.publication(&publication.slug)),
        "logo": publication.icon_url(urls, 512),
    })
}

pub fn publication_head(publication: &publication::Model, urls: &Urls) -> Markup {
    let url = urls.publication(&publication.slug);
    html! {
        meta property="og:site_name" content=(publication.name);
        link rel="alternate" type="application/rss+xml" title=(publication.name) href=(format!("{url}/feed"));
        link rel="apple-touch-icon" href=(publication.icon_url(urls, 180));
    }
}

pub fn newsletter(newsletter: Newsletter, ctx: &PageContext) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();
    let url = newsletter_url(&publication_url, &newsletter);
    let document = Document::from_stored(&newsletter.content);
    let content = render(&document);
    let published = newsletter.published_at.unwrap_or(newsletter.created_at);
    let date = published.format("%B %-d, %Y").to_string();

    let description = newsletter_description(&newsletter, &document);
    let image = newsletter_image(&document, &newsletter, publication, &ctx.urls);
    let metadata = Metadata::article(&description, &publication.name, &url)
        .image(&image)
        .modified_time(newsletter.updated_at.to_rfc3339());
    let entity = publication_entity(publication, &ctx.urls);
    let head = html! {
        (publication_head(publication, &ctx.urls))
        meta property="article:published_time" content=(published.to_rfc3339());
        link rel="alternate" type="text/markdown" href=(format!("{url}.md"));
        (json_ld(serde_json::json!({
            "@context": "https://schema.org",
            "@type": "Article",
            "headline": newsletter.display_title(),
            "description": description,
            "image": image,
            "url": url,
            "mainEntityOfPage": url,
            "datePublished": published.to_rfc3339(),
            "dateModified": newsletter.updated_at.to_rfc3339(),
            "author": entity,
            "publisher": entity,
        })))
    };

    shell(
        page(titled(newsletter.display_title(), publication))
            .favicon(publication.icon_url(&ctx.urls, 64))
            .htmx()
            .seo(metadata)
            .head(head),
        ctx,
        html! {
        main.article-layout .newsletter {
            article.prose .flow .prose-body {
                ArticleHeader(
                    date = &date,
                    title = newsletter.display_title(),
                    subtitle = [newsletter.subtitle.as_deref()],
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
                SubscribeForm(&publication_url, &publication.name);
            }
        }
    })
}
