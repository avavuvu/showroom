use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header::REFERER},
};
use boutique::{ErrorReport, html};
use maud::Markup;

use crate::{
    state::AppState,
    views::{PageContext, Viewer, pages::error::error_page},
};

pub async fn render(State(state): State<AppState>, viewer: Viewer, headers: HeaderMap, report: ErrorReport) -> Markup {
    let message = (!report.messages.is_empty()).then(|| {
        html! {
            @for (_, message) in &report.messages {
                p { (message) }
            }
        }
    });

    let back = (report.status == StatusCode::UNPROCESSABLE_ENTITY)
        .then(|| headers.get(REFERER).and_then(|value| value.to_str().ok()).filter(|value| !value.is_empty()))
        .flatten();

    let ctx = PageContext::public(viewer, state.urls.clone());
    error_page(&ctx, report.status, message, back)
}
