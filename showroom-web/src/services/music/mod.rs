mod apple_music;
mod bandcamp;
pub mod links;

use std::time::Duration;

use reqwest::redirect::{Attempt, Policy};
use serde::Serialize;

use crate::blocks::card::{clean_text, color};
use crate::blocks::music::{MusicKind, MusicService, is_allowed_artwork};
use crate::services::fetch::{self, USER_AGENT};

const TIMEOUT: Duration = Duration::from_secs(6);
const MAX_REDIRECTS: usize = 3;
const UNSUPPORTED: &str = "That isn't a song or album link. Music cards work with songs and albums from Apple Music, Spotify, Bandcamp and YouTube Music, not playlists or artists.";

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Details {
    pub title: String,
    pub artist: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub artwork: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub color: Option<String>,
}

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Resolved {
    pub service: MusicService,
    pub kind: MusicKind,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub id: Option<String>,
    #[ts(optional = false)]
    pub details: Option<Details>,
}

#[derive(Clone)]
pub struct MusicClient {
    client: reqwest::Client,
}

impl Default for MusicClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .user_agent(USER_AGENT)
            .redirect(Policy::custom(follow))
            .build()
            .expect("the music lookup client could not be built");
        Self { client }
    }

    pub async fn resolve(&self, input: &str) -> Result<Resolved, &'static str> {
        let link = links::parse(input).ok_or(UNSUPPORTED)?;
        let mut resolved = Resolved { service: link.service, kind: link.kind, url: link.url.clone(), id: link.id.clone(), details: None };

        match link.service {
            MusicService::AppleMusic => match apple_music::lookup(&self.client, &link).await {
                Ok(details) => resolved.details = details,
                Err(error) => tracing::warn!(%error, url = %link.url, "apple music lookup failed"),
            },
            MusicService::Bandcamp => match bandcamp::lookup(&self.client, &link).await {
                Ok(Some(page)) => {
                    resolved.kind = page.kind.unwrap_or(link.kind);
                    resolved.id = page.id;
                    resolved.details = Some(page.details);
                }
                Ok(None) => {}
                Err(error) => tracing::warn!(%error, url = %link.url, "bandcamp lookup failed"),
            },
            MusicService::Spotify | MusicService::YoutubeMusic => {}
        }

        resolved.details = resolved.details.map(clean).filter(|details| !details.title.is_empty() || !details.artist.is_empty());
        if let Some(details) = resolved.details.as_mut()
            && let Some(artwork) = details.artwork.as_deref()
        {
            details.color = self.color(artwork).await;
        }
        Ok(resolved)
    }

    pub async fn color(&self, artwork: &str) -> Option<String> {
        if !is_allowed_artwork(artwork) {
            return None;
        }
        fetch::image_color(&self.client, &color::small_artwork_url(artwork)).await
    }
}

fn clean(details: Details) -> Details {
    Details {
        title: clean_text(&details.title),
        artist: clean_text(&details.artist),
        artwork: details.artwork.filter(|artwork| is_allowed_artwork(artwork)),
        color: details.color,
    }
}

fn follow(attempt: Attempt) -> reqwest::redirect::Action {
    let allowed = attempt.url().scheme() == "https"
        && attempt
            .url()
            .host_str()
            .is_some_and(|host| host == "itunes.apple.com" || host == "bandcamp.com" || host.ends_with(".bandcamp.com"));
    if allowed && attempt.previous().len() < MAX_REDIRECTS { attempt.follow() } else { attempt.stop() }
}
