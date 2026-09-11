use axum_extra::extract::cookie::CookieJar;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::{
    cookies, jwt,
    models::{refresh_token::{self, Entity as RefreshToken}, user::{self, Entity as User}},
    password,
    state::AuthState,
};

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    Database(DbErr),
}

impl From<DbErr> for LoginError {
    fn from(e: DbErr) -> Self {
        LoginError::Database(e)
    }
}

#[derive(Debug)]
pub enum SessionError {
    Token(jsonwebtoken::errors::Error),
    Database(DbErr),
}

impl From<jsonwebtoken::errors::Error> for SessionError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        SessionError::Token(e)
    }
}

impl From<DbErr> for SessionError {
    fn from(e: DbErr) -> Self {
        SessionError::Database(e)
    }
}

pub async fn authenticate(state: &AuthState, email: &str, plain_password: &str) -> Result<user::Model, LoginError> {
    let user = User::find()
        .filter(user::Column::Email.eq(email))
        .one(&state.db)
        .await?;

    match user {
        Some(user) if password::verify(plain_password, &user.password) => Ok(user),
        _ => Err(LoginError::InvalidCredentials),
    }
}

pub fn new_user(email: &str, plain_password: &str) -> Result<user::ActiveModel, argon2::password_hash::Error> {
    Ok(user::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(email.to_string()),
        password: Set(password::hash(plain_password)?),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    })
}

pub async fn issue(state: &AuthState, jar: CookieJar, user: &user::Model) -> Result<CookieJar, SessionError> {
    let claims = jwt::Claims::new(&user.id, &user.email, state.config.jwt_ttl_hours);
    let jwt_token = jwt::generate(state.secret(), &claims)?;

    let refresh_value = uuid::Uuid::new_v4().to_string();
    refresh_token::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user.id.clone()),
        token: Set(refresh_value.clone()),
        expires_at: Set((chrono::Utc::now() + chrono::Duration::hours(state.config.refresh_ttl_hours)).into()),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&state.db)
    .await?;

    Ok(jar
        .add(cookies::make(cookies::JWT, jwt_token, state.config.jwt_ttl_hours, &state.config))
        .add(cookies::make(cookies::REFRESH, refresh_value, state.config.refresh_ttl_hours, &state.config)))
}

pub async fn revoke(state: &AuthState, jar: CookieJar) -> CookieJar {
    if let Some(refresh) = jar.get(cookies::REFRESH) {
        let _ = RefreshToken::delete_many()
            .filter(refresh_token::Column::Token.eq(refresh.value()))
            .exec(&state.db)
            .await;
    }

    jar
        .remove(cookies::remove(cookies::JWT, &state.config))
        .remove(cookies::remove(cookies::REFRESH, &state.config))
}
