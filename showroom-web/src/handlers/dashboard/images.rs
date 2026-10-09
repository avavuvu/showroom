use axum::{
    Json,
    extract::{Multipart, State},
    response::{IntoResponse, Redirect, Response},
};
use boutique::{AppResult, AuthenticatedUser};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Serialize;

use crate::{
    models::{publication, user},
    services::publication::OwnedPublication,
    state::AppState,
    views,
};

const ALLOWED_FORMATS: &str = "jpg,jpeg,png,gif,webp";
const IMAGE_TYPES: &[&str] = &["image/png", "image/jpeg", "image/gif", "image/webp"];
pub const MAX_IMAGE_BYTES: usize = 10 * 1024 * 1024;

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

struct ImageForm {
    choice: Option<String>,
    file: Option<(Vec<u8>, String)>,
}

async fn read_form(mut multipart: Multipart) -> Result<ImageForm, &'static str> {
    let mut form = ImageForm { choice: None, file: None };

    while let Some(field) = multipart.next_field().await.map_err(|_| "The upload could not be read. Images must be 10 MB or smaller.")? {
        match field.name() {
            Some("picture") => form.choice = field.text().await.ok(),
            Some("file") => {
                let content_type = field.content_type().unwrap_or_default().to_string();
                let bytes = field.bytes().await.map_err(|_| "The upload could not be read. Images must be 10 MB or smaller.")?;
                if bytes.is_empty() {
                    continue;
                }
                if !IMAGE_TYPES.contains(&content_type.as_str()) {
                    return Err("Images must be PNG, JPEG, GIF or WebP.");
                }
                form.file = Some((bytes.to_vec(), content_type));
            }
            _ => {}
        }
    }

    Ok(form)
}

async fn upload(state: &AppState, publication: &publication::Model, kind: &str, (bytes, content_type): (Vec<u8>, String)) -> Result<String, String> {
    let public_id = format!("showroom/publications/{}/{kind}-{}", publication.id, nanoid::nanoid!(10));
    state.cloudinary.upload(bytes, &public_id, &content_type).await.map_err(|e| {
        eprintln!("[images] {kind} upload failed for {}: {e}", publication.id);
        "The image could not be uploaded. Try again.".to_string()
    })
}

fn settings_url(state: &AppState, publication: &publication::Model) -> String {
    format!("{}/settings", state.urls.dashboard(&publication.slug))
}

fn retry(state: &AppState, owned: OwnedPublication, message: &str) -> Response {
    views::dashboard::publications::settings(&owned.into_context(state.urls.clone()), Some(message)).into_response()
}

pub async fn update_picture(State(state): State<AppState>, owned: OwnedPublication, multipart: Multipart) -> AppResult {
    let form = match read_form(multipart).await {
        Ok(form) => form,
        Err(message) => return Ok(retry(&state, owned, message)),
    };

    let pictures = owned.publication.pictures.clone();
    let (image, pictures) = match (form.file, form.choice) {
        (Some(file), _) => match upload(&state, &owned.publication, "picture", file).await {
            Ok(public_id) => (public_id.clone(), pictures.with_newest(public_id)),
            Err(message) => return Ok(retry(&state, owned, &message)),
        },
        (None, Some(choice)) => match publication::Picture::parse(&choice) {
            publication::Picture::Default(_) => (choice, pictures),
            publication::Picture::Upload(public_id) if pictures.contains(public_id) => (choice, pictures),
            publication::Picture::Upload(_) => return Ok(retry(&state, owned, "Choose one of the pictures.")),
        },
        (None, None) => return Ok(Redirect::to(&settings_url(&state, &owned.publication)).into_response()),
    };

    let url = settings_url(&state, &owned.publication);
    let mut active: publication::ActiveModel = owned.publication.into();
    active.image = Set(image);
    active.pictures = Set(pictures);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());
    active.update(&state.db).await?;

    Ok(Redirect::to(&url).into_response())
}

pub async fn update_banner(State(state): State<AppState>, owned: OwnedPublication, multipart: Multipart) -> AppResult {
    let file = match read_form(multipart).await {
        Ok(ImageForm { file: Some(file), .. }) => file,
        Ok(_) => return Ok(retry(&state, owned, "Choose an image for the banner.")),
        Err(message) => return Ok(retry(&state, owned, message)),
    };

    let banner = match upload(&state, &owned.publication, "banner", file).await {
        Ok(public_id) => public_id,
        Err(message) => return Ok(retry(&state, owned, &message)),
    };

    let url = settings_url(&state, &owned.publication);
    let mut active: publication::ActiveModel = owned.publication.into();
    active.banner = Set(Some(banner));
    active.updated_at = Set(chrono::Utc::now().fixed_offset());
    active.update(&state.db).await?;

    Ok(Redirect::to(&url).into_response())
}

pub async fn remove_banner(State(state): State<AppState>, owned: OwnedPublication) -> AppResult<Redirect> {
    let url = settings_url(&state, &owned.publication);
    let mut active: publication::ActiveModel = owned.publication.into();
    active.banner = Set(None);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());
    active.update(&state.db).await?;

    Ok(Redirect::to(&url))
}
