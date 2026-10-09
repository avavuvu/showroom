use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn wordmark(#[builder(default)] attrs: Attrs) -> Markup {
    html! {
        span.wordmark role="img" aria-label="Showroom" (..attrs) {}
    }
}
