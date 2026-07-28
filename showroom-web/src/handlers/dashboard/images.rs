use axum::{Json, extract::State};
use serde::Serialize;
use crate::{auth::extractors::AuthenticatedUser, state::AppState};

#[derive(Serialize)]
pub struct SignatureResponse {
    pub signature: String,
    pub timestamp: i64,
    pub api_key: String,
    pub cloud_name: String,
}

pub async fn sign_upload(
    State(state): State<AppState>,
    AuthenticatedUser(_): AuthenticatedUser,
) -> Json<SignatureResponse> {
    let timestamp = chrono::Utc::now().timestamp();
    let signature = state.cloudinary.sign(timestamp);

    Json(SignatureResponse {
        signature,
        timestamp,
        api_key: state.cloudinary.api_key.clone(),
        cloud_name: state.cloudinary.cloud_name.clone(),
    })
}
