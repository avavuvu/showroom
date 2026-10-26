mod editor;
mod embed;

pub use editor::YoutubeEditor;
pub use embed::YoutubeEmbed;

use maud::{Markup, PreEscaped};
use serde::{Deserialize, Deserializer, Serialize};

use crate::blocks::card::{self, clean_text, lenient_bool, null_string};
use crate::theme::Color;

const MAX_START: u32 = 12 * 60 * 60;

pub const WATCH_LABEL: &str = "Watch on YouTube";

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct YoutubeAttrs {
    #[serde(default, deserialize_with = "null_string")]
    pub id: String,
    #[serde(default, deserialize_with = "lenient_start", skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(default, deserialize_with = "lenient_bool", skip_serializing_if = "is_false")]
    pub short: bool,
    #[serde(default, deserialize_with = "lenient_bool", skip_serializing_if = "is_false")]
    pub hd: bool,
    #[serde(default, deserialize_with = "null_string")]
    pub title: String,
    #[serde(default, deserialize_with = "null_string")]
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl YoutubeAttrs {
    pub fn watch_url(&self) -> String {
        match self.start {
            Some(start) => format!("https://www.youtube.com/watch?v={}&t={start}s", self.id),
            None => format!("https://www.youtube.com/watch?v={}", self.id),
        }
    }

    pub fn embed_url(&self) -> String {
        match self.start {
            Some(start) => format!("https://www.youtube-nocookie.com/embed/{}?autoplay=1&start={start}", self.id),
            None => format!("https://www.youtube-nocookie.com/embed/{}?autoplay=1", self.id),
        }
    }

    pub fn thumbnail_url(&self) -> String {
        let quality = if self.hd { "maxresdefault" } else { "hqdefault" };
        format!("https://i.ytimg.com/vi/{}/{quality}.jpg", self.id)
    }

    pub fn surface(&self) -> Option<Color> {
        card::surface(self.color.as_deref())
    }

    pub fn has_details(&self) -> bool {
        !self.title.is_empty() || !self.channel.is_empty()
    }
}

pub fn normalize(attrs: YoutubeAttrs) -> Option<YoutubeAttrs> {
    let id = attrs.id.trim();
    if !is_valid_id(id) {
        return None;
    }
    Some(YoutubeAttrs {
        id: id.to_string(),
        start: attrs.start.filter(|start| (1..=MAX_START).contains(start)),
        short: attrs.short,
        hd: attrs.hd,
        title: clean_text(&attrs.title),
        channel: clean_text(&attrs.channel),
        channel_url: attrs.channel_url.map(|url| url.trim().to_string()).filter(|url| is_allowed_channel_url(url)),
        color: card::card_color(attrs.color),
    })
}

pub fn is_valid_id(id: &str) -> bool {
    id.len() == 11 && id.chars().all(|character| character.is_ascii_alphanumeric() || "-_".contains(character))
}

pub fn is_allowed_channel_url(href: &str) -> bool {
    url::Url::parse(href).is_ok_and(|url| {
        url.scheme() == "https"
            && url.username().is_empty()
            && url.password().is_none()
            && matches!(url.host_str(), Some("www.youtube.com" | "youtube.com" | "m.youtube.com"))
    })
}

fn is_false(value: &bool) -> bool {
    !value
}

fn play_badge() -> Markup {
    PreEscaped(
        r##"<svg class="youtube-play" viewBox="0 0 68 48" aria-hidden="true"><path class="youtube-play-shape" d="M66.5 7.7c-.8-2.9-2.5-5.4-5.4-6.2C55.8.1 34 0 34 0S12.2.1 6.9 1.6c-3 .7-4.6 3.2-5.4 6.1a89.6 89.6 0 0 0 0 32.5c.8 3 2.5 5.5 5.4 6.3C12.2 47.9 34 48 34 48s21.8-.1 27.1-1.6c3-.7 4.6-3.2 5.4-6.1C68 35 68 24 68 24s0-11-1.5-16.3z"/><path d="M45 24 27 14v20" fill="#fff"/></svg>"##
            .to_string(),
    )
}

fn lenient_start<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u32>, D::Error> {
    Ok(Option::<i64>::deserialize(deserializer).ok().flatten().and_then(|start| u32::try_from(start).ok()))
}
