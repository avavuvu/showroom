use sea_orm::DatabaseConnection;

use crate::config::AuthConfig;

#[derive(Clone)]
pub struct AuthState {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
    pub config: AuthConfig,
}

impl AuthState {
    pub fn new(db: DatabaseConnection, jwt_secret: impl Into<String>, config: AuthConfig) -> Self {
        Self { db, jwt_secret: jwt_secret.into(), config }
    }

    pub fn secret(&self) -> &[u8] {
        self.jwt_secret.as_bytes()
    }
}
