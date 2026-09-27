use axum::{Json, extract::State};
use boutique::{AuthenticatedUser, cloudinary::Signature};
use crate::state::AppState;

pub async fn sign_upload(
    State(state): State<AppState>,
    AuthenticatedUser(_): AuthenticatedUser,
) -> Json<Signature> {
    Json(state.cloudinary.signature())
}
