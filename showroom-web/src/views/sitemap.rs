use axum::{http::header, response::{IntoResponse, Response}};
use maud::{Markup, PreEscaped, html};

use crate::{
    models::{newsletter::Model as Newsletter, publication::Model as Publication},
    state::Urls,
};

pub struct Xml(Markup);

impl IntoResponse for Xml {
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
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

pub fn publications(publications: &[Publication], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for publication in publications {
                url { loc { (urls.publication(&publication.slug)) "/" } }
            }
        }
    })
}

pub fn newsletters(items: &[(Newsletter, Publication)], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for (newsletter, publication) in items {
                url {
                    loc { (urls.publication(&publication.slug)) "/" (newsletter.slug) }
                    @if let Some(sent_at) = newsletter.sent_at {
                        lastmod { (sent_at.format("%Y-%m-%d")) }
                    }
                }
            }
        }
    })
}
