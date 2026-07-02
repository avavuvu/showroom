use axum::{extract::State, Extension};
use maud::Markup;
use crate::{auth::context::UserContext, state::AppState, views::{self, PageContext}};

pub async fn index(State(state): State<AppState>, Extension(ctx): Extension<UserContext>) -> Markup {
    views::home::index(&PageContext::public(&ctx, state.urls.clone()))
}
