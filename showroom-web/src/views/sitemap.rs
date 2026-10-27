use axum::{http::header, response::{IntoResponse, Response}};
use boutique::html;
use maud::{Markup, PreEscaped};

use chrono::{DateTime, FixedOffset};

pub struct Entry {
    pub loc: String,
    pub lastmod: Option<DateTime<FixedOffset>>,
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

pub fn urlset(entries: &[Entry]) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for entry in entries {
                url {
                    loc { (entry.loc) }
                    @if let Some(lastmod) = &entry.lastmod {
                        lastmod { (lastmod.format("%Y-%m-%d")) }
                    }
                }
            }
        }
    })
}
