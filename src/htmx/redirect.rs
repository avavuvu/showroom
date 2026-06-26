use axum::{
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

pub fn redirect(path: &str) -> Response {
    (
        StatusCode::OK,
        [("HX-Redirect", HeaderValue::from_str(path).expect("invalid redirect path"))],
    )
        .into_response()
}
