use axum::extract::State;
use maud::Markup;
use crate::{state::AppState, views::{self, PageContext, Viewer}};

pub async fn index(State(state): State<AppState>, viewer: Viewer) -> Markup {
    views::home::index(&PageContext::public(viewer, state.urls.clone()))
}
