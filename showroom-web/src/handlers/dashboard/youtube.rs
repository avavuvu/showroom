use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use boutique::AuthenticatedUser;
use serde::{Deserialize, Serialize};

use crate::{models::user, state::AppState};

#[derive(Deserialize)]
pub struct LookupRequest {
    id: String,
}

#[derive(Serialize)]
struct LookupProblem {
    error: &'static str,
}

pub async fn lookup(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser<user::Model>,
    Json(body): Json<LookupRequest>,
) -> Response {
    match state.youtube.lookup(&body.id).await {
        Ok(details) => Json(details).into_response(),
        Err(error) => (StatusCode::UNPROCESSABLE_ENTITY, Json(LookupProblem { error })).into_response(),
    }
}
