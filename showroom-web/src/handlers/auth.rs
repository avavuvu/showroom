use axum::{
    Extension, Form, extract::{Query, State}, response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use boutique::{UserContext, htmx, reset, session::{self, LoginError}};
use validator::Validate;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;
use crate::{
    mailer, models::{publication::{self, Entity as Publication}, user::{self, Entity as User}}, state::AppState, views::{self, PageContext}
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

fn something_went_wrong() -> Response {
    htmx::fragments::error("Something went wrong, please try again").into_response()
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

    let user = match session::authenticate(&state.auth, &form.email, &form.password).await {
        Ok(user) => user,
        Err(LoginError::InvalidCredentials) => {
            return htmx::fragments::error("Incorrect email or password").into_response();
        }
        Err(LoginError::Database(e)) => {
            eprintln!("[login] {e}");
            return something_went_wrong();
        }
    };

    match session::issue(&state.auth, jar, &user).await {
        Ok(jar) => (jar, htmx::redirect(&state.urls.app())).into_response(),
        Err(e) => {
            eprintln!("[login] {e:?}");
            something_went_wrong()
        }
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

    match session::issue(&state.auth, jar, &user).await {
        Ok(jar) => (jar, htmx::redirect(&state.urls.app())).into_response(),
        Err(e) => {
            eprintln!("[signup] {e:?}");
            something_went_wrong()
        }
    }
}

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    let jar = session::revoke(&state.auth, jar).await;
    (jar, Redirect::to(&state.urls.base())).into_response()
}

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordForm {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
}

pub async fn forgot_password_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
) -> Response {
    views::auth::forgot_password(&PageContext::public(&ctx, state.urls.clone())).into_response()
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Form(form): Form<ForgotPasswordForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    match reset::request(&state.auth, &form.email).await {
        Ok(Some((user, token))) => {
            let reset_url = format!("{}/reset-password?token={}", state.urls.base(), token);
            if let Err(e) = mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls).await {
                eprintln!("[forgot-password] email failed for {}: {e}", user.email);
                return something_went_wrong();
            }
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("[forgot-password] {e:?}");
            return something_went_wrong();
        }
    }

    views::auth::forgot_password_sent().into_response()
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: String,
}

pub async fn reset_password_page(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    Query(params): Query<TokenQuery>,
) -> Response {
    let page_ctx = PageContext::public(&ctx, state.urls.clone());

    match reset::verify_token(&state.auth, &params.token).await {
        Ok(_) => views::auth::reset_password(&page_ctx, &params.token).into_response(),
        Err(_) => views::auth::reset_password_invalid(&page_ctx).into_response(),
    }
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordForm {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirm: String,
}

pub async fn reset_password(
    State(state): State<AppState>,
    Form(form): Form<ResetPasswordForm>,
) -> Response {
    if let Err(errors) = form.validate() {
        return htmx::oob_only(htmx::fragments::from_errors(errors));
    }

    match reset::complete(&state.auth, &form.token, &form.password).await {
        Ok(_) => htmx::redirect(&format!("{}/login", state.urls.base())),
        Err(reset::ResetError::InvalidToken) => {
            htmx::fragments::error("This link is invalid or has already been used").into_response()
        }
        Err(e) => {
            eprintln!("[reset-password] {e:?}");
            something_went_wrong()
        }
    }
}
