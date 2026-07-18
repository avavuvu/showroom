use axum::response::{IntoResponse, Response};
use maud::Markup;

pub fn oob_only(markup: Markup) -> Response {
    ([("HX-Reswap", "none")], markup).into_response()
}
