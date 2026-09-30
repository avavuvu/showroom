use axum::{Json, extract::State};
use boutique::{AuthenticatedUser, cloudinary::Signature};
use crate::{models::user, state::AppState};

pub async fn sign_upload(
    State(state): State<AppState>,
    AuthenticatedUser(_): AuthenticatedUser<user::Model>,
) -> Json<Signature> {
    Json(state.cloudinary.signature())
}
