use axum::{extract::{Extension, State}, routing::{MethodRouter, get}};
use maud::Markup;
use boutique::UserContext;
use crate::{state::AppState, views::PageContext};

pub fn passthrough(view: fn(&PageContext) -> Markup) -> MethodRouter<AppState> {
    get(move |State(state): State<AppState>, Extension(ctx): Extension<UserContext>| async move {
        let page_ctx = PageContext::public(&ctx, state.urls.clone());
        view(&page_ctx)
    })
}
