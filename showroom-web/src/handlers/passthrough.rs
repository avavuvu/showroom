use axum::{extract::State, routing::{MethodRouter, get}};
use maud::Markup;
use crate::{state::AppState, views::{PageContext, Viewer}};

pub fn passthrough(view: fn(&PageContext) -> Markup) -> MethodRouter<AppState> {
    get(move |State(state): State<AppState>, viewer: Viewer| async move {
        let page_ctx = PageContext::public(viewer, state.urls.clone());
        view(&page_ctx)
    })
}
