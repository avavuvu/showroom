use axum::{
    body::{Body, to_bytes},
    extract::{OptionalFromRequestParts, Request, State},
    http::{
        HeaderMap,
        header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use boutique::{AuthenticatedUser, html};
use maud::PreEscaped;

use crate::{
    models::user,
    state::AppState,
    views::{PageContext, pages::error::error_page},
};

const BODY_LIMIT: usize = 64 * 1024;

pub async fn error_pages(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let headers = request.headers().clone();
    let response = next.run(request).await;
    let status = response.status();

    if !(status.is_client_error() || status.is_server_error()) || !wants_page(&headers) {
        return response;
    }

    let (parts, body) = response.into_parts();
    let bytes = to_bytes(body, BODY_LIMIT).await.unwrap_or_default();
    let text = String::from_utf8_lossy(&bytes).trim().to_string();

    if text.get(..9).is_some_and(|start| start.eq_ignore_ascii_case("<!doctype")) {
        return Response::from_parts(parts, Body::from(bytes));
    }

    let is_html = parts
        .headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("text/html"));

    let message = match text.is_empty() {
        true => None,
        false if is_html => Some(PreEscaped(text)),
        false => Some(html! { p { (text) } }),
    };

    let ctx = PageContext::public(viewer(headers, &state).await, state.urls.clone());
    let mut page = (status, error_page(&ctx, status, message)).into_response();

    for (name, value) in parts.headers.iter().filter(|(name, _)| *name != CONTENT_TYPE && *name != CONTENT_LENGTH) {
        page.headers_mut().append(name.clone(), value.clone());
    }

    page
}

fn wants_page(headers: &HeaderMap) -> bool {
    !headers.contains_key("hx-request")
        && headers
            .get(ACCEPT)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.contains("text/html"))
}

async fn viewer(headers: HeaderMap, state: &AppState) -> Option<AuthenticatedUser<user::Model>> {
    let (mut parts, ()) = Request::new(()).into_parts();
    parts.headers = headers;
    <AuthenticatedUser<user::Model> as OptionalFromRequestParts<AppState>>::from_request_parts(&mut parts, state)
        .await
        .ok()
        .flatten()
}
