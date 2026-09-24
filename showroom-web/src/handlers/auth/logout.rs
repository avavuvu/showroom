use axum::{extract::State, response::{IntoResponse, Redirect, Response}};
use boutique::session::{self, CookieJar};

use crate::state::AppState;

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    let session = session::revoke(&state.auth, jar).await;
    (session, Redirect::to(&state.urls.base())).into_response()
}
