use boutique::prelude::*;
use crate::{components::{ArticleHeader, SubscribeForm}, document::{Block, Document, ImageAttrs, is_allowed_image_src}, models::{newsletter::Model as Newsletter, publication::{self, Picture}}, renderer::html::render, state::{Urls}, views::{context::PageContext, layouts::{Metadata, page, shell}}};
pub mod profile;

fn with_share_image(metadata: Metadata, publication: &publication::Model, urls: &Urls) -> Metadata {
    match (publication.banner_url(urls), publication.picture()) {
        (Some(banner), _) => metadata.image(&banner),
        (None, Picture::Upload(_)) => metadata.image(publication.picture_url(urls)),
        (None, Picture::Default(_)) => metadata,
    }
}



pub fn get_hero_image(nodes: &[Block]) -> Option<String> {
    first_image(nodes).map(|image| image.src.clone())
}

pub fn first_image(blocks: &[Block]) -> Option<&ImageAttrs> {
    blocks.iter().find_map(|block| match block {
        Block::Image { attrs } if is_allowed_image_src(&attrs.src) => Some(attrs),
        Block::Blockquote { content, .. }
        | Block::Pullquote { content }
        | Block::Figure { content, .. } => first_image(content),
        Block::BulletList { content } | Block::OrderedList { content, .. } => {
            content.iter().find_map(|item| first_image(&item.content))
        }
        _ => None,
    })
}

pub fn newsletter(newsletter: Newsletter, ctx: &PageContext) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();
    let content = render(&Document::from_stored(&newsletter.content));
    let date = newsletter.published_at.unwrap_or(newsletter.created_at).format("%B %-d, %Y").to_string();

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
