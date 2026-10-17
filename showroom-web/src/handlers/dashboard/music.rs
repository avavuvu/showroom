use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use boutique::AuthenticatedUser;
use serde::{Deserialize, Serialize};

use crate::{models::user, state::AppState};

const MAX_URL_LENGTH: usize = 2048;

#[derive(Deserialize)]
pub struct ResolveRequest {
    url: String,
}

#[derive(Serialize)]
struct ResolveProblem {
    error: &'static str,
}

#[derive(Deserialize)]
pub struct ColorRequest {
    artwork: String,
}

#[boutique::ts]
#[derive(Serialize)]
pub struct ArtworkColor {
    #[ts(optional = false)]
    color: Option<String>,
}

pub async fn resolve(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser<user::Model>,
    Json(body): Json<ResolveRequest>,
) -> Response {
    if body.url.len() > MAX_URL_LENGTH {
        return problem("That link is too long.");
    }
    match state.music.resolve(&body.url).await {
        Ok(resolved) => Json(resolved).into_response(),
        Err(error) => problem(error),
    }
}

pub async fn color(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser<user::Model>,
    Json(body): Json<ColorRequest>,
) -> Json<ArtworkColor> {
    let color = if body.artwork.len() > MAX_URL_LENGTH { None } else { state.music.color(&body.artwork).await };
    Json(ArtworkColor { color })
}

fn problem(error: &'static str) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(ResolveProblem { error })).into_response()
}
