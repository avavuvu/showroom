use axum::response::Html;
use crate::views;

pub async fn index() -> Html<String> {
    Html(views::home::index())
}
