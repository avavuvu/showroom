use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use sea_orm::EntityTrait;

use crate::{
    context::UserContext,
    models::user::{self, Entity as User},
    state::AuthState,
};

pub struct AuthenticatedUser(pub user::Model);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = AuthState::from_ref(state);
        let to_login = || Redirect::to(&auth.config.login_url).into_response();

        let user_id = parts
            .extensions
            .get::<UserContext>()
            .and_then(|ctx| ctx.user_id.clone())
            .ok_or_else(to_login)?;

        let user = User::find_by_id(&user_id)
            .one(&auth.db)
            .await
            .map_err(|_| to_login())?
            .ok_or_else(to_login)?;

        Ok(AuthenticatedUser(user))
    }
}
