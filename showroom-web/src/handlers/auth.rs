use axum::{
    Extension, Form, extract::State, response::{IntoResponse, Redirect, Response},
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use axum_extra::extract::cookie::CookieJar;
use validator::Validate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::Deserialize;
use crate::{
    auth::{context::UserContext, cookies, jwt}, htmx, models::{publication::{self, Entity as Publication}, refresh_token, user::{self, Entity as User}}, state::AppState, views::{self, PageContext}
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
pub struct LoginForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
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

pub async fn login_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> Response {
    if ctx.is_authenticated() {
        return Redirect::to(&state.urls.app()).into_response();
    }

    views::auth::login(&PageContext::public(&ctx, state.urls.clone())).into_response()
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

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    let user = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await
        .unwrap();

    let valid = user.as_ref().map_or(false, |u| {
        PasswordHash::new(&u.password)
            .map(|hash| Argon2::default().verify_password(form.password.as_bytes(), &hash).is_ok())
            .unwrap_or(false)
    });

    if !valid {
        return htmx::fragments::error("Incorrect email or password").into_response();
    }

    let user = user.unwrap();

    match issue_session(&state, jar, &user).await {
        Ok(jar) => (jar, htmx::redirect(&state.urls.app())).into_response(),
        Err(response) => response,
    }
}

pub async fn signup(
    State(state): State<AppState>,
    jar: CookieJar,
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

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let now = chrono::Utc::now().fixed_offset();
    let user_id = uuid::Uuid::new_v4().to_string();

    let new_user = user::ActiveModel {
        id: Set(user_id.clone()),
        email: Set(form.email.clone()),
        password: Set(hash),
        created_at: Set(now),
        ..Default::default()
    };

    let default_room = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: Set(user_id),
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
            return htmx::fragments::error("Something went wrong, please try again").into_response();
        }
    };

    match issue_session(&state, jar, &user).await {
        Ok(jar) => (jar, htmx::redirect(&state.urls.app())).into_response(),
        Err(response) => response,
    }
}

async fn issue_session(state: &AppState, jar: CookieJar, user: &user::Model) -> Result<CookieJar, Response> {
    let claims = jwt::Claims::new(&user.email, &user.id);
    let jwt_token = jwt::generate(state.jwt_secret.as_bytes(), claims)
        .map_err(|_| htmx::fragments::error("Something went wrong, please try again").into_response())?;

    let refresh_token_value = uuid::Uuid::new_v4().to_string();
    let new_refresh = refresh_token::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id.clone()),
        token: Set(refresh_token_value.clone()),
        expires_at: Set((chrono::Utc::now() + chrono::Duration::days(30)).into()),
        created_at: Set(chrono::Utc::now().into()),
    };
    new_refresh.insert(&state.db).await.unwrap();

    Ok(jar
        .add(cookies::make("jwt", jwt_token, 1, &state.urls.cookie()))
        .add(cookies::make("refresh", refresh_token_value, 30 * 24, &state.urls.cookie())))
}

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    let jar = jar
        .remove(cookies::remove("jwt", &state.urls.cookie()))
        .remove(cookies::remove("refresh", &state.urls.cookie()));
    (jar, Redirect::to(&state.urls.base())).into_response()
}
