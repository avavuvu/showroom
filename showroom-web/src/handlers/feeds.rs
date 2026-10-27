use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, header},
    response::{IntoResponse, Response},
};
use boutique::AppResult;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Select};

use crate::{
    document::Document,
    models::{newsletter::{self, Entity as Newsletter, Model as NewsletterModel}, publication},
    renderer::{html::render, markdown},
    services::subdomain::CurrentPublication,
    state::AppState,
    views::user::{newsletter_description, newsletter_url, publication_description},
};

const FEED_LENGTH: u64 = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Html,
    Markdown,
}

impl Format {
    pub fn negotiate<'a>(slug: &'a str, headers: &HeaderMap) -> (&'a str, Format) {
        if let Some(slug) = slug.strip_suffix(".md") {
            return (slug, Format::Markdown);
        }
        let accept = headers.get(header::ACCEPT).and_then(|value| value.to_str().ok()).unwrap_or_default();
        let quality = |target: &str| {
            accept
                .split(',')
                .filter_map(|part| {
                    let mut params = part.split(';');
                    let mime = params.next()?.trim();
                    let quality = params.find_map(|param| param.trim().strip_prefix("q=")).and_then(|q| q.parse::<f32>().ok()).unwrap_or(1.0);
                    mime.eq_ignore_ascii_case(target).then_some(quality)
                })
                .fold(0.0, f32::max)
        };
        let markdown = quality("text/markdown");
        if markdown > 0.0 && markdown > quality("text/html") { (slug, Format::Markdown) } else { (slug, Format::Html) }
    }
}

fn text(content_type: &'static str, cache: &'static str, body: String) -> Response {
    ([(header::CONTENT_TYPE, content_type), (header::CACHE_CONTROL, cache)], body).into_response()
}

fn plain(body: String) -> Response {
    text("text/plain; charset=utf-8", "public, max-age=3600", body)
}

fn yaml(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

pub fn markdown_response(publication: &publication::Model, newsletter: &NewsletterModel, url: &str) -> Response {
    let document = Document::from_stored(&newsletter.content);
    let published = newsletter.published_at.unwrap_or(newsletter.created_at);
    let mut body = format!("---\ntitle: {}\n", yaml(newsletter.display_title()));
    if let Some(subtitle) = newsletter.subtitle.as_deref().map(str::trim).filter(|subtitle| !subtitle.is_empty()) {
        body.push_str(&format!("subtitle: {}\n", yaml(subtitle)));
    }
    body.push_str(&format!("author: {}\ndate: {}\nurl: {}\n---\n\n# {}\n\n{}\n", yaml(&publication.name), published.format("%Y-%m-%d"), yaml(url), newsletter.display_title(), markdown::render(&document)));

    let mut response = text("text/markdown; charset=utf-8", "public, max-age=300", body);
    let headers = response.headers_mut();
    headers.insert(header::VARY, HeaderValue::from_static("Accept"));
    if let Ok(link) = HeaderValue::from_str(&format!("<{url}>; rel=\"canonical\"")) {
        headers.insert(header::LINK, link);
    }
    response
}

pub fn html_headers(response: &mut Response, url: &str) {
    let headers = response.headers_mut();
    headers.insert(header::VARY, HeaderValue::from_static("Accept"));
    if let Ok(link) = HeaderValue::from_str(&format!("<{url}.md>; rel=\"alternate\"; type=\"text/markdown\"")) {
        headers.insert(header::LINK, link);
    }
}

pub async fn site_robots(State(state): State<AppState>) -> Response {
    plain(format!("User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n", state.urls.base()))
}

pub async fn app_robots() -> Response {
    plain("User-agent: *\nDisallow: /\n".to_string())
}

pub async fn publication_robots(State(state): State<AppState>, CurrentPublication(publication): CurrentPublication) -> Response {
    plain(format!(
        "User-agent: *\nDisallow: /api/\nDisallow: /confirm\nDisallow: /unsubscribe\n\nSitemap: {}/sitemap.xml\n",
        state.urls.publication(&publication.slug)
    ))
}

fn published(query: Select<Newsletter>, publication: &publication::Model) -> Select<Newsletter> {
    query
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .order_by_desc(newsletter::Column::PublishedAt)
}

fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn cdata(value: &str) -> String {
    format!("<![CDATA[{}]]>", value.replace("]]>", "]]]]><![CDATA[>"))
}

pub async fn rss(State(state): State<AppState>, CurrentPublication(publication): CurrentPublication) -> AppResult<Response> {
    let newsletters = published(Newsletter::find(), &publication).limit(FEED_LENGTH).all(&state.db).await?;
    let url = state.urls.publication(&publication.slug);

    let mut items = String::new();
    for newsletter in &newsletters {
        let document = Document::from_stored(&newsletter.content);
        let link = newsletter_url(&url, newsletter);
        let published = newsletter.published_at.unwrap_or(newsletter.created_at);
        items.push_str(&format!(
            "<item><title>{}</title><link>{}</link><guid isPermaLink=\"true\">{}</guid><pubDate>{}</pubDate><description>{}</description><content:encoded>{}</content:encoded></item>",
            escape(newsletter.display_title()),
            escape(&link),
            escape(&link),
            published.to_rfc2822(),
            escape(&newsletter_description(newsletter, &document)),
            cdata(&render(&document).into_string()),
        ));
    }

    let last_build = newsletters.iter().map(|newsletter| newsletter.updated_at).max().unwrap_or(publication.updated_at);
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\"><channel><title>{name}</title><link>{url}/</link><description>{description}</description><language>en</language><lastBuildDate>{last_build}</lastBuildDate><atom:link href=\"{url}/feed\" rel=\"self\" type=\"application/rss+xml\"/><image><url>{image}</url><title>{name}</title><link>{url}/</link></image>{items}</channel></rss>",
        name = escape(&publication.name),
        url = escape(&url),
        description = escape(&publication_description(&publication)),
        last_build = last_build.to_rfc2822(),
        image = escape(&publication.email_picture_url(&state.urls)),
    );
    Ok(text("application/rss+xml; charset=utf-8", "public, max-age=600", body))
}

pub async fn llms(State(state): State<AppState>, CurrentPublication(publication): CurrentPublication) -> AppResult<Response> {
    let newsletters = published(Newsletter::find_summaries(), &publication).all(&state.db).await?;
    let url = state.urls.publication(&publication.slug);
    let mut body = format!("# {}\n\n> {}\n\n## Newsletters\n\n", publication.name, publication_description(&publication));
    for newsletter in &newsletters {
        body.push_str(&format!("- [{}]({}.md)", newsletter.display_title(), newsletter_url(&url, newsletter)));
        if let Some(subtitle) = newsletter.subtitle.as_deref().map(str::trim).filter(|subtitle| !subtitle.is_empty()) {
            body.push_str(&format!(": {subtitle}"));
        }
        body.push('\n');
    }
    body.push_str(&format!("\n## Optional\n\n- [RSS feed]({url}/feed)\n"));
    Ok(text("text/markdown; charset=utf-8", "public, max-age=3600", body))
}
