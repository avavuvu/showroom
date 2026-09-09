use maud::{Markup, PreEscaped, html};
use crate::{models::newsletter::Model as Newsletter, renderer::html::render, views::{components::forms::subscribe::subscribe_form, context::PageContext, layouts::{Metadata, ViewContext, shell}}};

pub fn profile(ctx: &PageContext, newsletters: &[Newsletter]) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();

    let metadata = Metadata::article(
        publication.description.as_deref().unwrap_or(&publication.name),
        &publication.name,
        &publication_url);

    shell(
        ViewContext::page(&publication.name)
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
    let html_string = render(&newsletter.content);
    let date = newsletter.created_at.format("%B %-d, %Y").to_string();

    let metadata = Metadata::article(
        &newsletter.title,
        &publication.name,
        &publication_url);

    shell(
        ViewContext::page(&newsletter.title)
            .htmx()
            .seo(metadata),
        ctx,
        html! {
        main.article-layout .newsletter {
            article.prose .flow {
                div.info {
                    p.date { (date) }
                    h1 { (newsletter.title) }
                    @if let Some(subtitle) = &newsletter.subtitle {
                        p.subtitle { (subtitle) }
                    }
                    p.handle {
                        a href=(publication_url) { (publication.name) }
                    }
                }
                (PreEscaped(html_string))
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
                    a href=(format!("{}/{}", publication_url, newsletter.slug)) { (newsletter.title) }
                }
            }
        }
    }
}
