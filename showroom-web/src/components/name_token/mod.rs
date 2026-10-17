use boutique::html;
use bq_components::component;
use maud::Markup;

const TITLE: &str = "The subscriber's name";

#[component]
pub fn name_token(fallback: Option<&str>) -> Markup {
    html! {
        span.name-token data-name="" data-fallback=[fallback] contenteditable="false" title=(TITLE) {
            "name"
            @if let Some(fallback) = fallback {
                span.name-token-fallback { (fallback) }
            }
        }
    }
}
