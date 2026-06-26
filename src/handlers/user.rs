use axum::{extract::State, Extension};
use maud::Markup;
use crate::{auth::context::UserContext, state::AppState, services::subdomain::UsernameSubdomain, views};

pub async fn profile(
    State(state): State<AppState>,
    UsernameSubdomain(username): UsernameSubdomain,
    Extension(ctx): Extension<UserContext>,
) -> Markup {
    views::user::profile(&username, ctx.is_authenticated(), &state.urls.base)
}
