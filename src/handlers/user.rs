use axum::{extract::State, response::Html};
use crate::{auth::Auth, state::AppState, services::subdomain::UsernameSubdomain, views};

pub async fn profile(
    State(state): State<AppState>,
    UsernameSubdomain(username): UsernameSubdomain,
    auth: Auth,
) -> Html<String> {
    Html(views::user::profile(&username, auth.is_authenticated(), &state.urls.base))
}
