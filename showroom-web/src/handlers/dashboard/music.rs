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

fn problem(error: &'static str) -> Response {
    (StatusCode::UNPROCESSABLE_ENTITY, Json(ResolveProblem { error })).into_response()
}
