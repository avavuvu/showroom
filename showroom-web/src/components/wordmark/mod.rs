use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn wordmark(attrs: Attrs) -> Markup {
    html! {
        span.wordmark role="img" aria-label="Showroom" (..attrs) {}
    }
}
