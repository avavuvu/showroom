use axum::{http::header, response::{IntoResponse, Response}};
use maud::{Markup, PreEscaped, html};

use crate::{
    models::{newsletter::Model as Newsletter, user::Model as User},
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
            sitemap { loc { (base) "/sitemap-users.xml" } }
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

pub fn users(users: &[User], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for user in users {
                url { loc { (urls.user(&user.handle)) "/" } }
            }
        }
    })
}

pub fn newsletters(items: &[(Newsletter, User)], urls: &Urls) -> Xml {
    Xml(html! {
        (decl())
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            @for (newsletter, user) in items {
                url {
                    loc { (urls.user(&user.handle)) "/" (newsletter.slug) }
                    @if let Some(sent_at) = newsletter.sent_at {
                        lastmod { (sent_at.format("%Y-%m-%d")) }
                    }
                }
            }
        }
    })
}
