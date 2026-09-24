use axum::{Extension, Form, extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::{UserContext, htmx, session};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use super::something_went_wrong;
use crate::{
    models::{publication::{self, Entity as Publication}, user::{self, Entity as User}},
    state::AppState,
    views::{self, PageContext},
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
    Extension(ctx): Extension<UserContext>,
) -> Response {
    if ctx.is_authenticated() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::signup(&PageContext::public(&ctx, state.urls.clone())).into_response()
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    let slug = form.handle.to_lowercase();
    if let Err(message) = publication::validate_slug(&slug) {
        return htmx::fragments::field_errors(&[("handle", Some(message))]).into_response();
    }

    let email_taken = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    let handle_taken = Publication::find()
        .filter(publication::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    if email_taken || handle_taken {
        return htmx::fragments::field_errors(&[
            ("email", email_taken.then_some("An account with this email already exists")),
            ("handle", handle_taken.then_some("This handle is already taken")),
        ]).into_response();
    }

    let new_user = match session::new_user(&form.email, &form.password) {
        Ok(user) => user,
        Err(e) => {
            eprintln!("[signup] {e}");
            return something_went_wrong();
        }
    };

    let now = chrono::Utc::now().fixed_offset();
    let default_room = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: new_user.id.clone(),
        slug: Set(slug.clone()),
        name: Set(format!("{slug}'s room")),
        description: Set(None),
        theme: Set(None),
        is_default: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = match state.db.transaction::<_, user::Model, sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let user = new_user.insert(txn).await?;
            default_room.insert(txn).await?;
            Ok(user)
        })
    }).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("[signup] {e}");
            return something_went_wrong();
        }
    };

    match session::issue(&state.auth, &user).await {
        Ok(session) => (session, htmx::redirect(&state.urls.app())).into_response(),
        Err(e) => {
            eprintln!("[signup] {e:?}");
            something_went_wrong()
        }
    }
}
