use axum::{extract::State, response::Html};
use crate::{auth::Auth, state::AppState, views};

pub async fn index(State(state): State<AppState>, auth: Auth) -> Html<String> {
    Html(views::dashboard::index(auth.is_authenticated(), &state.urls.base))
}
