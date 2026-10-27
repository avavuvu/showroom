use boutique::prelude::*;
use crate::{components::SubscribeForm, models::newsletter::Model as Newsletter, {images::ImageSize, state::Urls}, views::{PageContext, layouts::{Metadata, page, shell}, user::{json_ld, publication_description, publication_entity, publication_head, publication_image}}};

pub fn index(ctx: &PageContext, newsletters: &[Newsletter]) -> Markup {
    let publication = ctx.publication();
    let publication_url = ctx.publication_url();

    let description = publication_description(publication);
    let metadata = Metadata::article(&description, &publication.name, &publication_url).image(publication_image(publication, &ctx.urls));
    let mut entity = publication_entity(publication, &ctx.urls);
    entity["description"] = description.into();
    let head = html! {
        (publication_head(publication, &ctx.urls))
        @if newsletters.is_empty() {
            meta name="robots" content="noindex";
        }
        (json_ld(serde_json::json!({
            "@context": "https://schema.org",
            "@type": "ProfilePage",
            "url": format!("{publication_url}/"),
            "mainEntity": entity,
        })))
    };

    let content = html! {
        @if let Some((lead, secondary, archive)) = front_page(newsletters) {
            div.front data-layout="magazine" {
                LeadStory(lead, &publication_url, &ctx.urls);

                @if !secondary.is_empty() {
                    SecondaryStories(secondary, &publication_url, &ctx.urls);
                }
            }

            div.subscribe {
                p {
                    "To recieve updates whenever "
                    a href=(publication_url) { (publication.name) }
                    " posts, consider subscribing."
                }
                SubscribeForm(&publication_url, &publication.name);
            }

            @if !archive.is_empty() {
                div.front data-layout="magazine" {
                    ArchiveList(archive, &publication_url, &ctx.urls);
                }
            }

        } @else {
            div.empty {
                p {
                    "To receive updates when " span.publication-name { (&publication.name) } " first posts, consider subscribing."
                }

                SubscribeForm(&publication_url, &publication.name);
            }
        }
    };

    shell(
        page(&publication.name).favicon(publication.icon_url(&ctx.urls, 64)).htmx().seo(metadata).head(head),
        ctx,
        html! {
            main.profile-view {
                div.bio {
                    div.thumb {
                        img.publication-picture src=(publication.picture_url(&ctx.urls)) alt="";
                    }
                    h1 { (publication.name) }
                    @if let Some(description) = &publication.description {
                        p.description { (description) }
                    }

                }
                (content)
            }
        },
    )
}

fn front_page(newsletters: &[Newsletter]) -> Option<(&Newsletter, &[Newsletter], &[Newsletter])> {
    let (lead, rest) = newsletters.split_first()?;
    let (secondary, archive) = rest.split_at(rest.len().min(2));
    Some((lead, secondary, archive))
}

fn story_href(publication_url: &str, newsletter: &Newsletter) -> String {
    format!("{}/{}", publication_url, newsletter.slug)
}

fn dateline(newsletter: &Newsletter) -> Markup {
    html! {
        @if let Some(date) = newsletter.published_at {
            time.dateline datetime=(date.to_rfc3339()) { (date.format("%B %-d, %Y")) }
        }
    }
}

#[component]
fn lead_story(#[positional] newsletter: &Newsletter, #[positional] publication_url: &str, #[positional] urls: &Urls) -> Markup {
    let href = story_href(publication_url, newsletter);
    html! {
        article.lead {
            @if let Some(public_id) = &newsletter.hero_image_id {
                a.thumb href=(href) tabindex="-1" aria-hidden="true" {
                    img src=(urls.image(public_id, ImageSize::Hero)) width="1600" height="900" alt="";
                }
            }
            hgroup {
                h2 { a href=(href) { (newsletter.display_title()) } }
                @if let Some(subtitle) = &newsletter.subtitle {
                    p { (subtitle) }
                }
                (dateline(newsletter))
            }
        }
    }
}

#[component]
fn secondary_stories(#[positional] newsletters: &[Newsletter], #[positional] publication_url: &str, #[positional] urls: &Urls) -> Markup {
    html! {
        ul.secondary {
            @for newsletter in newsletters {
                @let href = story_href(publication_url, newsletter);
                li {
                    article {
                        @if let Some(public_id) = &newsletter.hero_image_id {
                            a.thumb href=(href) tabindex="-1" aria-hidden="true" {
                                img src=(urls.image(public_id, ImageSize::Card)) width="640" height="360" loading="lazy" decoding="async" alt="";
                            }
                        }
                        h3 { a href=(href) { (newsletter.display_title()) } }
                        @if let Some(subtitle) = &newsletter.subtitle {
                            p { (subtitle) }
                        }
                        (dateline(newsletter))
                    }
                }
            }
        }
    }
}

#[component]
fn archive_list(#[positional] newsletters: &[Newsletter], #[positional] publication_url: &str, #[positional] urls: &Urls) -> Markup {
    html! {
        section.archive {
            ul {
                @for newsletter in newsletters {
                    @let href = story_href(publication_url, newsletter);
                    li {
                        div.text {
                            h3 { a href=(href) { (newsletter.display_title()) } }
                            @if let Some(subtitle) = &newsletter.subtitle {
                                p { (subtitle) }
                            }
                            (dateline(newsletter))
                        }
                        @if let Some(public_id) = &newsletter.hero_image_id {
                            a.thumb href=(href) tabindex="-1" aria-hidden="true" {
                                img src=(urls.image(public_id, ImageSize::Card)) width="640" height="360" loading="lazy" decoding="async" alt="";
                            }
                        }
                    }
                }
            }
        }
    }
}
