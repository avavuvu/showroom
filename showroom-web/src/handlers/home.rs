use axum::{extract::State, Extension};
use maud::Markup;
use boutique::UserContext;
use crate::{state::AppState, views::{self, PageContext}};

pub async fn index(State(state): State<AppState>, Extension(ctx): Extension<UserContext>) -> Markup {
    views::home::index(&PageContext::public(&ctx, state.urls.clone()))
}
