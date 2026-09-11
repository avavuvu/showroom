use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

use crate::{
    models::{publication, user},
    state::{AppState, Urls},
    views::PageContext,
};

pub use boutique::AuthenticatedUser;

pub struct OwnedPublication {
    pub user: user::Model,
    pub publication: publication::Model,
    pub publications: Vec<publication::Model>,
}

impl OwnedPublication {
    pub fn into_context(self, urls: Urls) -> PageContext {
        PageContext::from_user(&self.user, urls)
            .with_publications(self.publications)
            .with_publication(self.publication)
    }
}

impl FromRequestParts<AppState> for OwnedPublication {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let AuthenticatedUser(user) = AuthenticatedUser::from_request_parts(parts, state).await?;

        let Path(params) = Path::<HashMap<String, String>>::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::NOT_FOUND.into_response())?;
        let slug = params.get("slug").ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

        let publications = publication::for_owner(&user.id, &state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        let publication = publications
            .iter()
            .find(|p| &p.slug == slug)
            .cloned()
            .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;

        Ok(OwnedPublication { user, publication, publications })
    }
}
