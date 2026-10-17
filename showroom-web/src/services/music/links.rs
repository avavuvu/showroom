use url::Url;

use crate::blocks::music::{MusicKind, MusicService, is_valid_music_id};

const DEFAULT_COUNTRY: &str = "us";
const YOUTUBE_ALBUM_PREFIX: &str = "OLAK5uy_";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicUrl {
    pub service: MusicService,
    pub kind: MusicKind,
    pub id: Option<String>,
    pub url: String,
    pub country: String,
}

impl MusicUrl {
    fn new(service: MusicService, kind: MusicKind, id: Option<String>, url: String) -> MusicUrl {
        MusicUrl { service, kind, id, url, country: DEFAULT_COUNTRY.to_string() }
    }
}

pub fn parse(input: &str) -> Option<MusicUrl> {
    let input = input.trim();
    if let Some(rest) = input.strip_prefix("spotify:") {
        return spotify_uri(rest);
    }

    let url = Url::parse(input).ok()?;
    if !matches!(url.scheme(), "https" | "http") {
        return None;
    }
    let host = url.host_str()?.to_string();
    let segments: Vec<&str> = url.path_segments().map(|segments| segments.filter(|segment| !segment.is_empty()).collect()).unwrap_or_default();

    match host.as_str() {
        "open.spotify.com" => spotify(&segments),
        "music.apple.com" | "geo.music.apple.com" => apple_music(&url, &segments),
        "music.youtube.com" => youtube_music(&url, &segments),
        _ if host.ends_with(".bandcamp.com") => bandcamp(&host, &segments),
        _ => None,
    }
}

fn spotify_kind(segment: &str) -> Option<MusicKind> {
    match segment {
        "track" => Some(MusicKind::Track),
        "album" => Some(MusicKind::Album),
        _ => None,
    }
}

fn spotify_link(kind: MusicKind, id: &str) -> Option<MusicUrl> {
    if !is_valid_music_id(id) || !id.chars().all(|character| character.is_ascii_alphanumeric()) {
        return None;
    }
    let url = format!("https://open.spotify.com/{}/{id}", kind.as_str());
    Some(MusicUrl::new(MusicService::Spotify, kind, Some(id.to_string()), url))
}

fn spotify(segments: &[&str]) -> Option<MusicUrl> {
    let segments = match segments.first() {
        Some(first) if first.starts_with("intl-") => &segments[1..],
        _ => segments,
    };
    match segments {
        [kind, id, ..] => spotify_link(spotify_kind(kind)?, id),
        _ => None,
    }
}

fn spotify_uri(rest: &str) -> Option<MusicUrl> {
    let (kind, id) = rest.split_once(':')?;
    spotify_link(spotify_kind(kind)?, id)
}

fn numeric_id(segment: &str) -> Option<String> {
    let id = segment.strip_prefix("id").unwrap_or(segment);
    (!id.is_empty() && id.len() <= 20 && id.chars().all(|character| character.is_ascii_digit())).then(|| id.to_string())
}

fn apple_music(url: &Url, segments: &[&str]) -> Option<MusicUrl> {
    let country = segments
        .first()
        .filter(|first| first.len() == 2 && first.chars().all(|character| character.is_ascii_alphabetic()))
        .map(|first| first.to_ascii_lowercase());
    let position = segments.iter().position(|segment| matches!(*segment, "album" | "song"))?;
    let item_id = numeric_id(segments.last()?)?;
    let song_id = url.query_pairs().find(|(name, _)| name == "i").and_then(|(_, value)| numeric_id(&value));

    let (kind, id) = match (segments[position], song_id.clone()) {
        ("album", Some(song_id)) => (MusicKind::Track, song_id),
        ("album", None) => (MusicKind::Album, item_id),
        _ => (MusicKind::Track, item_id),
    };

    let mut canonical = format!("https://music.apple.com/{}", segments.join("/"));
    if let Some(song_id) = song_id {
        canonical.push_str("?i=");
        canonical.push_str(&song_id);
    }

    Some(MusicUrl {
        service: MusicService::AppleMusic,
        kind,
        id: Some(id),
        url: canonical,
        country: country.unwrap_or_else(|| DEFAULT_COUNTRY.to_string()),
    })
}

fn youtube_music(url: &Url, segments: &[&str]) -> Option<MusicUrl> {
    let query = |key: &str| url.query_pairs().find(|(name, _)| name == key).map(|(_, value)| value.into_owned());
    match segments {
        ["watch"] => {
            let id = query("v").filter(|id| id.len() == 11 && is_valid_music_id(id))?;
            let canonical = format!("https://music.youtube.com/watch?v={id}");
            Some(MusicUrl::new(MusicService::YoutubeMusic, MusicKind::Track, Some(id), canonical))
        }
        ["playlist"] => {
            let id = query("list").filter(|id| id.starts_with(YOUTUBE_ALBUM_PREFIX) && is_valid_music_id(id))?;
            let canonical = format!("https://music.youtube.com/playlist?list={id}");
            Some(MusicUrl::new(MusicService::YoutubeMusic, MusicKind::Album, Some(id), canonical))
        }
        _ => None,
    }
}

fn bandcamp(host: &str, segments: &[&str]) -> Option<MusicUrl> {
    let [kind, slug, ..] = segments else { return None };
    let kind = match *kind {
        "track" => MusicKind::Track,
        "album" => MusicKind::Album,
        _ => return None,
    };
    if slug.is_empty() || slug.len() > 200 || !slug.chars().all(|character| character.is_ascii_alphanumeric() || "-_".contains(character)) {
        return None;
    }
    let canonical = format!("https://{host}/{}/{slug}", segments[0]);
    Some(MusicUrl::new(MusicService::Bandcamp, kind, None, canonical))
}
