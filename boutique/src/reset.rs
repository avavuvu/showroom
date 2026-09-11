use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::{
    jwt,
    models::user::{self, Entity as User},
    password,
    state::AuthState,
};

#[derive(Debug)]
pub enum ResetError {
    InvalidToken,
    Database(DbErr),
    Hash(argon2::password_hash::Error),
}

impl From<DbErr> for ResetError {
    fn from(e: DbErr) -> Self {
        ResetError::Database(e)
    }
}

/// returns `None` for an unknown email. callers must show the same message
/// either way so the endpoint does not reveal which emails have accounts.
pub async fn request(state: &AuthState, email: &str) -> Result<Option<(user::Model, String)>, ResetError> {
    let Some(user) = User::find()
        .filter(user::Column::Email.eq(email))
        .one(&state.db)
        .await?
    else {
        return Ok(None);
    };

    let token = token_for(state, &user).map_err(|_| ResetError::InvalidToken)?;
    Ok(Some((user, token)))
}

pub fn token_for(state: &AuthState, user: &user::Model) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = jwt::PasswordResetClaims::new(&user.id, &user.password, state.config.reset_ttl_hours);
    jwt::generate_password_reset(state.secret(), &claims)
}

pub async fn verify_token(state: &AuthState, token: &str) -> Result<user::Model, ResetError> {
    let claims = jwt::validate_password_reset(state.secret(), token).map_err(|_| ResetError::InvalidToken)?;

    let user = User::find_by_id(claims.user_id())
        .one(&state.db)
        .await?
        .ok_or(ResetError::InvalidToken)?;

    if jwt::hash_fragment(&user.password) != claims.password_hash_fragment {
        return Err(ResetError::InvalidToken);
    }

    Ok(user)
}

pub async fn complete(state: &AuthState, token: &str, new_password: &str) -> Result<user::Model, ResetError> {
    let user = verify_token(state, token).await?;

    let mut active: user::ActiveModel = user.into();
    active.password = Set(password::hash(new_password).map_err(ResetError::Hash)?);
    active.updated_at = Set(Some(chrono::Utc::now().into()));

    Ok(active.update(&state.db).await?)
}
