pub mod fragments;

use axum::{
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use maud::Markup;

pub fn redirect(path: &str) -> Response {
    (
        StatusCode::OK,
        [("HX-Redirect", HeaderValue::from_str(path).expect("invalid redirect path"))],
    )
        .into_response()
}

pub fn oob_only(markup: Markup) -> Response {
    ([("HX-Reswap", "none")], markup).into_response()
}
