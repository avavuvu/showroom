use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use sea_orm::EntityTrait;
use tower_sessions::Session;
use crate::{
    models::user::{self, Entity as User},
    state::AppState,
};

pub struct Auth {
    pub user: Option<user::Model>,
    pub session: Session,
}

impl Auth {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub async fn login(&self, user_id: &str) -> Result<(), tower_sessions::session::Error> {
        self.session.insert("user_id", user_id).await
    }

    pub async fn logout(&self) -> Result<(), tower_sessions::session::Error> {
        self.session.flush().await
    }
}

impl FromRequestParts<AppState> for Auth {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let user_id: Option<String> = session
            .get("user_id")
            .await
            .ok()
            .flatten();

        let user = if let Some(id) = user_id {
            User::find_by_id(id)
                .one(&state.db)
                .await
                .ok()
                .flatten()
        } else {
            None
        };

        Ok(Auth { user, session })
    }
}
