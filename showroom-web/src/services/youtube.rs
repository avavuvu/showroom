use std::time::Duration;

use reqwest::{StatusCode, redirect::Policy};
use serde::{Deserialize, Serialize};

use crate::blocks::card::{card_color, clean_text};
use crate::blocks::youtube::{is_allowed_channel_url, is_valid_id};
use crate::services::fetch::{self, USER_AGENT};

const TIMEOUT: Duration = Duration::from_secs(6);
const NOT_FOUND: &str = "That video could not be found. It may be private or removed.";
const FAILED: &str = "The video details could not be looked up.";

#[boutique::ts]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct YoutubeDetails {
    pub title: String,
    pub channel: String,
    #[ts(optional = false)]
    pub channel_url: Option<String>,
    #[ts(optional = false)]
    pub color: Option<String>,
    pub short: bool,
    pub hd: bool,
}

#[derive(Deserialize)]
struct Oembed {
    #[serde(default)]
    title: String,
    #[serde(default)]
    author_name: String,
    #[serde(default)]
    author_url: Option<String>,
    #[serde(default)]
    width: u32,
    #[serde(default)]
    height: u32,
}

#[derive(Clone)]
pub struct YoutubeClient {
    client: reqwest::Client,
}

impl Default for YoutubeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubeClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .user_agent(USER_AGENT)
            .redirect(Policy::none())
            .build()
            .expect("the youtube lookup client could not be built");
        Self { client }
    }

    pub async fn lookup(&self, id: &str) -> Result<YoutubeDetails, &'static str> {
        if !is_valid_id(id) {
            return Err(NOT_FOUND);
        }
        let (oembed, hd, color) = tokio::join!(self.oembed(id), self.has_hd_thumbnail(id), self.color(id));
        let oembed = oembed?;
        Ok(YoutubeDetails {
            title: clean_text(&oembed.title),
            channel: clean_text(&oembed.author_name),
            channel_url: oembed.author_url.filter(|url| is_allowed_channel_url(url)),
            color: card_color(color),
            short: oembed.height > oembed.width,
            hd,
        })
    }

    async fn oembed(&self, id: &str) -> Result<Oembed, &'static str> {
        let url = format!("https://www.youtube.com/oembed?format=json&url=https%3A%2F%2Fwww.youtube.com%2Fshorts%2F{id}");
        let response = self.client.get(url).send().await.map_err(|error| {
            tracing::warn!(%error, %id, "youtube oembed lookup failed");
            FAILED
        })?;
        match response.status() {
            StatusCode::OK => response.json::<Oembed>().await.map_err(|_| FAILED),
            StatusCode::NOT_FOUND | StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN | StatusCode::BAD_REQUEST => Err(NOT_FOUND),
            status => {
                tracing::warn!(%status, %id, "youtube oembed lookup failed");
                Err(FAILED)
            }
        }
    }

    async fn has_hd_thumbnail(&self, id: &str) -> bool {
        let url = format!("https://i.ytimg.com/vi/{id}/maxresdefault.jpg");
        self.client.head(url).send().await.is_ok_and(|response| response.status() == StatusCode::OK)
    }

    async fn color(&self, id: &str) -> Option<String> {
        fetch::image_color(&self.client, &format!("https://i.ytimg.com/vi/{id}/mqdefault.jpg")).await
    }
}
