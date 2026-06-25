use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::{auth::Auth, state::AppState, views};
use crate::models::user::{self, Entity as User};

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupForm {
    pub email: String,
    pub password: String,
}

pub async fn login_page() -> Html<String> {
    Html(views::auth::login(None))
}

pub async fn signup_page() -> Html<String> {
    Html(views::auth::signup(None))
}

pub async fn login(
    State(state): State<AppState>,
    auth: Auth,
    Form(form): Form<LoginForm>,
) -> Response {
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

    if valid {
        let user = user.unwrap();
        auth.login(&user.id).await.unwrap();
        Redirect::to("/").into_response()
    } else {
        Html(views::auth::login(Some("Invalid email or password"))).into_response()
    }
}

pub async fn signup(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> Response {
    let exists = User::find()
        .filter(user::Column::Email.eq(&form.email))
        .one(&state.db)
        .await
        .unwrap()
        .is_some();

    if exists {
        return Html(views::auth::signup(Some("An account with that email already exists"))).into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = user::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(form.email),
        password: Set(hash),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    new_user.insert(&state.db).await.unwrap();

    Redirect::to("/login").into_response()
}

pub async fn logout(auth: Auth) -> Response {
    auth.logout().await.unwrap();
    Redirect::to("/login").into_response()
}
