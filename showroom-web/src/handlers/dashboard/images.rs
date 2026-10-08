use axum::{Json, extract::State};
use boutique::AuthenticatedUser;
use serde::Serialize;

use crate::{models::user, state::AppState};

const ALLOWED_FORMATS: &str = "jpg,jpeg,png,gif,webp";

#[derive(Serialize)]
pub struct UploadSignature {
    signature: String,
    timestamp: i64,
    api_key: String,
    cloud_name: String,
    folder: String,
    allowed_formats: &'static str,
}

pub async fn sign_upload(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> Json<UploadSignature> {
    let timestamp = chrono::Utc::now().timestamp();
    let folder = format!("showroom/{}", user.id);
    let signature = state.cloudinary.sign(&[
        ("allowed_formats", ALLOWED_FORMATS),
        ("folder", &folder),
        ("timestamp", &timestamp.to_string()),
    ]);

    Json(UploadSignature {
        signature,
        timestamp,
        api_key: state.cloudinary.api_key().to_string(),
        cloud_name: state.cloudinary.cloud_name().to_string(),
        folder,
        allowed_formats: ALLOWED_FORMATS,
    })
}
