use serde::Deserialize;

use super::{Details, links::MusicUrl};
use crate::blocks::music::MusicKind;

const LOOKUP_URL: &str = "https://itunes.apple.com/lookup";

#[derive(Deserialize)]
struct LookupResponse {
    #[serde(default)]
    results: Vec<LookupItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LookupItem {
    wrapper_type: Option<String>,
    track_name: Option<String>,
    collection_name: Option<String>,
    artist_name: Option<String>,
    artwork_url100: Option<String>,
}

pub async fn lookup(client: &reqwest::Client, link: &MusicUrl) -> Result<Option<Details>, reqwest::Error> {
    let Some(id) = link.id.as_deref() else { return Ok(None) };
    let response: LookupResponse = client
        .get(LOOKUP_URL)
        .query(&[("id", id), ("country", link.country.as_str())])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let wanted = match link.kind {
        MusicKind::Track => "track",
        MusicKind::Album => "collection",
    };
    let Some(item) = response.results.into_iter().find(|item| item.wrapper_type.as_deref() == Some(wanted)) else {
        return Ok(None);
    };

    let title = match link.kind {
        MusicKind::Track => item.track_name,
        MusicKind::Album => item.collection_name,
    };
    Ok(Some(Details {
        title: title.unwrap_or_default(),
        artist: item.artist_name.unwrap_or_default(),
        artwork: item.artwork_url100.map(|artwork| artwork.replace("100x100bb", "600x600bb")),
    }))
}
