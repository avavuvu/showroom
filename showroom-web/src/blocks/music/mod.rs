mod card;
mod editor;
mod lookup;

pub use card::MusicCard;
pub use editor::MusicEditor;
pub use lookup::MusicLookup;

use serde::{Deserialize, Deserializer, Serialize};

use crate::blocks::card::{card_color, clean_text, null_string, surface};
use crate::theme::Color;

const ARTWORK_HOST_SUFFIXES: &[&str] = &[".mzstatic.com", ".bcbits.com"];
const ARTWORK_HOSTS: &[&str] = &["res.cloudinary.com"];

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicAttrs {
    #[serde(default, deserialize_with = "lenient_kind")]
    pub kind: MusicKind,
    #[serde(default, deserialize_with = "null_string")]
    pub title: String,
    #[serde(default, deserialize_with = "null_string")]
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artwork: Option<String>,
    #[serde(default, deserialize_with = "lenient_service", skip_serializing_if = "Option::is_none")]
    pub source: Option<MusicService>,
    #[serde(default, deserialize_with = "lenient_links")]
    pub links: MusicLinks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[boutique::ts]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum MusicKind {
    #[default]
    Track,
    Album,
}

#[boutique::ts]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MusicService {
    AppleMusic,
    Spotify,
    Bandcamp,
    YoutubeMusic,
}

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apple_music: Option<MusicLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spotify: Option<MusicLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bandcamp: Option<MusicLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube_music: Option<MusicLink>,
}

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicLink {
    #[serde(default, deserialize_with = "null_string")]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}


impl MusicKind {
    pub fn as_str(self) -> &'static str {
        match self {
            MusicKind::Track => "track",
            MusicKind::Album => "album",
        }
    }
}

impl MusicService {
    pub const ALL: [MusicService; 4] = [MusicService::AppleMusic, MusicService::Spotify, MusicService::Bandcamp, MusicService::YoutubeMusic];

    pub fn as_str(self) -> &'static str {
        match self {
            MusicService::AppleMusic => "appleMusic",
            MusicService::Spotify => "spotify",
            MusicService::Bandcamp => "bandcamp",
            MusicService::YoutubeMusic => "youtubeMusic",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MusicService::AppleMusic => "Apple Music",
            MusicService::Spotify => "Spotify",
            MusicService::Bandcamp => "Bandcamp",
            MusicService::YoutubeMusic => "YouTube Music",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            MusicService::AppleMusic => "apple-music",
            MusicService::Spotify => "spotify",
            MusicService::Bandcamp => "bandcamp",
            MusicService::YoutubeMusic => "youtube-music",
        }
    }

    fn allows_host(self, host: &str) -> bool {
        match self {
            MusicService::AppleMusic => matches!(host, "music.apple.com" | "geo.music.apple.com"),
            MusicService::Spotify => host == "open.spotify.com",
            MusicService::Bandcamp => host == "bandcamp.com" || host.ends_with(".bandcamp.com"),
            MusicService::YoutubeMusic => host == "music.youtube.com",
        }
    }

    fn parse(name: &str) -> Option<MusicService> {
        MusicService::ALL.into_iter().find(|service| service.as_str() == name)
    }
}

impl MusicLinks {
    pub fn get(&self, service: MusicService) -> Option<&MusicLink> {
        match service {
            MusicService::AppleMusic => self.apple_music.as_ref(),
            MusicService::Spotify => self.spotify.as_ref(),
            MusicService::Bandcamp => self.bandcamp.as_ref(),
            MusicService::YoutubeMusic => self.youtube_music.as_ref(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (MusicService, &MusicLink)> {
        MusicService::ALL.into_iter().filter_map(|service| self.get(service).map(|link| (service, link)))
    }

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

impl MusicAttrs {
    pub fn primary(&self) -> Option<(MusicService, &MusicLink)> {
        self.source
            .and_then(|service| self.links.get(service).map(|link| (service, link)))
            .or_else(|| self.links.iter().next())
    }

    pub fn surface(&self) -> Option<Color> {
        surface(self.color.as_deref())
    }
}

pub fn listen_label(service: MusicService) -> String {
    format!("Listen on {}", service.label())
}

pub fn normalize(attrs: MusicAttrs) -> Option<MusicAttrs> {
    let MusicAttrs { kind, title, artist, artwork, source, links, color } = attrs;
    let links = MusicLinks {
        apple_music: normalize_link(MusicService::AppleMusic, links.apple_music),
        spotify: normalize_link(MusicService::Spotify, links.spotify),
        bandcamp: normalize_link(MusicService::Bandcamp, links.bandcamp),
        youtube_music: normalize_link(MusicService::YoutubeMusic, links.youtube_music),
    };
    if links.is_empty() {
        return None;
    }

    Some(MusicAttrs {
        kind,
        title: clean_text(&title),
        artist: clean_text(&artist),
        artwork: artwork.map(|artwork| artwork.trim().to_string()).filter(|artwork| is_allowed_artwork(artwork)),
        source: source.filter(|service| links.get(*service).is_some()),
        links,
        color: card_color(color),
    })
}

fn normalize_link(service: MusicService, link: Option<MusicLink>) -> Option<MusicLink> {
    let link = link?;
    let url = link.url.trim().to_string();
    if !is_allowed_music_link(service, &url) {
        return None;
    }
    let id = link.id.map(|id| id.trim().to_string()).filter(|id| is_valid_music_id(id));
    Some(MusicLink { url, id })
}

pub fn is_allowed_music_link(service: MusicService, href: &str) -> bool {
    url::Url::parse(href).is_ok_and(|url| {
        url.scheme() == "https"
            && url.username().is_empty()
            && url.password().is_none()
            && url.host_str().is_some_and(|host| service.allows_host(host))
    })
}

pub fn is_allowed_artwork(src: &str) -> bool {
    url::Url::parse(src).is_ok_and(|url| {
        url.scheme() == "https"
            && url.host_str().is_some_and(|host| {
                ARTWORK_HOSTS.contains(&host) || ARTWORK_HOST_SUFFIXES.iter().any(|suffix| host.ends_with(suffix))
            })
    })
}

pub fn is_valid_music_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 128 && id.chars().all(|character| character.is_ascii_alphanumeric() || "-_".contains(character))
}

fn lenient_kind<'de, D: Deserializer<'de>>(deserializer: D) -> Result<MusicKind, D::Error> {
    Ok(match Option::<String>::deserialize(deserializer)?.as_deref() {
        Some("album") => MusicKind::Album,
        _ => MusicKind::Track,
    })
}

fn lenient_service<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<MusicService>, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.as_deref().and_then(MusicService::parse))
}

fn lenient_links<'de, D: Deserializer<'de>>(deserializer: D) -> Result<MusicLinks, D::Error> {
    Option::<MusicLinks>::deserialize(deserializer).map(Option::unwrap_or_default)
}
