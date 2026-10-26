use std::time::Duration;

use crate::blocks::card::color;

pub const USER_AGENT: &str = "Showroom/1.0 (+https://show.room.lc)";
const IMAGE_TIMEOUT: Duration = Duration::from_secs(3);
const MAX_IMAGE_BYTES: usize = 256 * 1024;

pub async fn read_limited(mut response: reqwest::Response, limit: usize) -> Result<Vec<u8>, reqwest::Error> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        let room = limit.saturating_sub(body.len());
        body.extend_from_slice(&chunk[..chunk.len().min(room)]);
        if body.len() >= limit {
            break;
        }
    }
    Ok(body)
}

pub async fn image_color(client: &reqwest::Client, url: &str) -> Option<String> {
    let bytes = async {
        let response = client.get(url).timeout(IMAGE_TIMEOUT).send().await?.error_for_status()?;
        read_limited(response, MAX_IMAGE_BYTES).await
    };
    match bytes.await {
        Ok(bytes) if bytes.len() < MAX_IMAGE_BYTES => color::from_image(&bytes).map(|color| color.to_string()),
        Ok(_) => None,
        Err(error) => {
            tracing::warn!(%error, %url, "image colour lookup failed");
            None
        }
    }
}
