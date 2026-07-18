use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use sea_orm::EntityTrait;

use crate::{
    auth::context::UserContext,
    models::user::{self, Entity as User},
    state::AppState,
};

pub struct AuthenticatedUser(pub user::Model);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let redirect_url = format!("{}/login", state.urls.base());

        let ctx = parts
            .extensions
            .get::<UserContext>()
            .and_then(|ctx| ctx.user_id.as_ref())
            .ok_or_else(|| Redirect::to(&redirect_url).into_response())?;

        let user = User::find_by_id(ctx)
            .one(&state.db)
            .await
            .map_err(|_| Redirect::to(&redirect_url).into_response())?
            .ok_or_else(|| Redirect::to(&redirect_url).into_response())?;

        Ok(AuthenticatedUser(user))
    }
}
