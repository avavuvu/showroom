use axum::{Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{AppError, AppResult, htmx, session};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::{
    models::{publication::{self, Entity as Publication}, user::{self, Entity as User}},
    state::AppState,
    views::{self, PageContext, Viewer},
};

fn alphanumeric(value: &str) -> Result<(), validator::ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        let mut e = validator::ValidationError::new("alphanumeric");
        e.message = Some("Handle can only contain letters and numbers".into());
        Err(e)
    }
}

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(
        custom(function = "alphanumeric", message = "Handle can only contain letters and numbers"),
        length(min = 3, max = 20, message = "Handle must be between 3 and 20 characters")
    )]
    pub handle: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[cfg(debug_assertions)]
pub async fn signup_page(
    State(state): State<AppState>,
    viewer: Viewer,
) -> Response {
    if viewer.is_some() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::signup(&PageContext::public(viewer, state.urls.clone())).into_response()
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> AppResult {
    form.validate()?;

    let slug = form.handle.to_lowercase();
    if let Err(message) = publication::validate_slug(&slug) {
        return Err(AppError::field("handle", message));
    }

    let email_taken = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await?
        .is_some();

    let handle_taken = Publication::find()
        .filter(publication::Column::Slug.eq(&slug))
        .one(&state.db)
        .await?
        .is_some();

    if email_taken || handle_taken {
        let mut fields = Vec::new();
        if email_taken {
            fields.push(("email", "An account with this email already exists".to_string()));
        }
        if handle_taken {
            fields.push(("handle", "This handle is already taken".to_string()));
        }
        return Err(AppError::Fields(fields));
    }

    let new_user = user::new(&form.email, &form.password)
        .map_err(|e| AppError::internal("signup password hash", e))?;

    let now = chrono::Utc::now().fixed_offset();
    let default_room = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: new_user.id.clone(),
        slug: Set(slug.clone()),
        name: Set(format!("{slug}'s room")),
        description: Set(None),
        theme: Set(None),
        greeting: Set(crate::renderer::greeting::DEFAULT.to_string()),
        image: Set(publication::random_picture()),
        banner: Set(None),
        pictures: Set(Default::default()),
        is_default: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = state.db.transaction::<_, user::Model, sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let user = new_user.insert(txn).await?;
            default_room.insert(txn).await?;
            Ok(user)
        })
    }).await.map_err(|e| AppError::internal("signup", e))?;

    let session = session::issue(&state.auth, &user)
        .await
        .map_err(|e| AppError::internal("signup session", e))?;

    Ok((session, htmx::redirect(&state.urls.app())).into_response())
}
