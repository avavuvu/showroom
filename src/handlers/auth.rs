use axum::{
    Form, extract::State, response::{IntoResponse, Redirect, Response},
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use axum_extra::extract::cookie::CookieJar;
use garde::Validate;
use maud::Markup;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::{
    auth::{cookies, jwt},
    htmx,
    models::{refresh_token, user::{self, Entity as User}},
    state::AppState,
};

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 1))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[garde(email)]
    pub email: String,
    #[garde(alphanumeric, length(min = 3, max = 20))]
    pub handle: String,
    #[garde(length(min = 8))]
    pub password: String,
}

pub async fn login_page() -> Markup {
    crate::views::auth::login(None)
}

pub async fn signup_page() -> Markup {
    crate::views::auth::signup(None)
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Response {
    if let Err(report) = form.validate() {
        return htmx::fragments::from_report(report).into_response();
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
        return htmx::fragments::error("Invalid email or password").into_response();
    }

    let user = user.unwrap();

    let claims = jwt::Claims::new(&user.email, &user.id);
    let jwt_token = match jwt::generate(state.jwt_secret.as_bytes(), claims) {
        Ok(t) => t,
        Err(_) => return htmx::fragments::error("Something went wrong, please try again").into_response(),
    };

    let refresh_token_value = uuid::Uuid::new_v4().to_string();
    let new_refresh = refresh_token::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        token: Set(refresh_token_value.clone()),
        expires_at: Set((chrono::Utc::now() + chrono::Duration::days(30)).into()),
        created_at: Set(chrono::Utc::now().into()),
    };
    new_refresh.insert(&state.db).await.unwrap();

    let jar = jar
        .add(cookies::make("jwt", jwt_token, 1, &state.urls.cookie()))
        .add(cookies::make("refresh", refresh_token_value, 30 * 24, &state.urls.cookie()));

    (jar, htmx::redirect("/")).into_response()
}

pub async fn signup(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<SignupForm>,
) -> Response {
    if let Err(report) = form.validate() {
        return htmx::fragments::from_report(report).into_response();
    }

    let email_taken = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    let handle_taken = User::find()
        .filter(user::Column::Handle.eq(&form.handle))
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

    let new_user = user::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(form.email.clone()),
        handle: Set(form.handle.clone()),
        password: Set(hash),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };
    let user = new_user.insert(&state.db).await.unwrap();

    let claims = jwt::Claims::new(&user.email, &user.id);
    let jwt_token = match jwt::generate(state.jwt_secret.as_bytes(), claims) {
        Ok(t) => t,
        Err(_) => return htmx::fragments::error("Something went wrong, please try again").into_response(),
    };

    let refresh_token_value = uuid::Uuid::new_v4().to_string();
    let new_refresh = refresh_token::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id),
        token: Set(refresh_token_value.clone()),
        expires_at: Set((chrono::Utc::now() + chrono::Duration::days(30)).into()),
        created_at: Set(chrono::Utc::now().into()),
    };
    new_refresh.insert(&state.db).await.unwrap();

    let jar = jar
        .add(cookies::make("jwt", jwt_token, 1, &state.urls.cookie()))
        .add(cookies::make("refresh", refresh_token_value, 30 * 24, &state.urls.cookie()));

    (jar, htmx::redirect("/login")).into_response()
}

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    let jar = jar
        .remove(cookies::remove("jwt", &state.urls.cookie()))
        .remove(cookies::remove("refresh", &state.urls.cookie()));
    (jar, Redirect::to(&state.urls.base())).into_response()
}
