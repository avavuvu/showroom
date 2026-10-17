use serde::Deserialize;

use super::{Details, links::MusicUrl, read_limited};
use crate::blocks::music::MusicKind;

const MAX_PAGE_BYTES: usize = 1024 * 1024;
const ARTWORK_SIZE: &str = "10";

pub struct Page {
    pub details: Details,
    pub kind: Option<MusicKind>,
    pub id: Option<String>,
}

#[derive(Deserialize)]
struct PageProperties {
    item_type: Option<String>,
    item_id: Option<u64>,
}

pub async fn lookup(client: &reqwest::Client, link: &MusicUrl) -> Result<Option<Page>, reqwest::Error> {
    let response = client.get(&link.url).send().await?.error_for_status()?;
    let html = read_limited(response, MAX_PAGE_BYTES).await?;
    Ok(page(&String::from_utf8_lossy(&html)))
}

fn page(html: &str) -> Option<Page> {
    let og_title = meta(html, "og:title")?;
    let (title, artist) = match og_title.rsplit_once(", by ") {
        Some((title, artist)) => (title.trim().to_string(), artist.trim().to_string()),
        None => (og_title.trim().to_string(), String::new()),
    };
    let properties = meta(html, "bc-page-properties").and_then(|json| serde_json::from_str::<PageProperties>(&json).ok());
    let kind = properties.as_ref().and_then(|properties| match properties.item_type.as_deref() {
        Some("a") => Some(MusicKind::Album),
        Some("t") => Some(MusicKind::Track),
        _ => None,
    });

    Some(Page {
        details: Details { title, artist, artwork: meta(html, "og:image").map(|artwork| larger_artwork(&artwork)), color: None },
        kind,
        id: properties.and_then(|properties| properties.item_id).map(|id| id.to_string()),
    })
}

fn meta(html: &str, key: &str) -> Option<String> {
    ["property", "name"].iter().find_map(|attribute| {
        let at = html.find(&format!("{attribute}=\"{key}\""))?;
        let start = html[..at].rfind('<')?;
        let end = at + html[at..].find('>')?;
        let content = attribute_value(&html[start..end], "content")?;
        Some(html_escape::decode_html_entities(content).trim().to_string())
    })
}

fn attribute_value<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(" {name}=");
    let rest = &tag[tag.find(&needle)? + needle.len()..];
    let quote = rest.chars().next().filter(|quote| matches!(quote, '"' | '\''))?;
    let rest = &rest[1..];
    Some(&rest[..rest.find(quote)?])
}

fn larger_artwork(src: &str) -> String {
    if let Some((stem, extension)) = src.rsplit_once('.')
        && let Some((base, size)) = stem.rsplit_once('_')
        && !size.is_empty()
        && size.chars().all(|character| character.is_ascii_digit())
    {
        return format!("{base}_{ARTWORK_SIZE}.{extension}");
    }
    src.to_string()
}
