use axum::{http::header, response::{IntoResponse, Response}};
use boutique::html;
use maud::{Markup, PreEscaped};

use chrono::{DateTime, FixedOffset};

use crate::state::Urls;

pub struct NewsletterEntry {
    pub publication_slug: String,
    pub slug: String,
    pub updated_at: DateTime<FixedOffset>,
}

pub struct Xml(Markup);

impl IntoResponse for Xml {
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8"), (header::CACHE_CONTROL, "public, max-age=3600")],
            self.0.into_string(),
        )
            .into_response()
    }
}

fn decl() -> Markup {
    PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string())
}

pub fn index(base: &str) -> Xml {
    Xml(html! {
        (decl())
        sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            sitemap { loc { (base) "/sitemap-pages.xml" } }
            sitemap { loc { (base) "/sitemap-publications.xml" } }
            sitemap { loc { (base) "/sitemap-newsletters.xml" } }
        }
    })
}

pub fn pages(base: &str) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            url { loc { (base) "/" } }
            url { loc { (base) "/about" } }
        }
    })
}

pub fn publications(slugs: &[String], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for slug in slugs {
                url { loc { (urls.publication(slug)) "/" } }
            }
        }
    })
}

pub fn newsletters(entries: &[NewsletterEntry], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for entry in entries {
                url {
                    loc { (urls.publication(&entry.publication_slug)) "/" (entry.slug) }
                    lastmod { (entry.updated_at.format("%Y-%m-%d")) }
                }
            }
        }
    })
}
