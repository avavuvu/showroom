use axum::{extract::State, Extension};
use maud::Markup;
use crate::{auth::context::UserContext, state::AppState, views};

pub async fn index(State(state): State<AppState>, Extension(ctx): Extension<UserContext>) -> Markup {
    views::dashboard::index(ctx.is_authenticated(), &state.urls.base)
}
