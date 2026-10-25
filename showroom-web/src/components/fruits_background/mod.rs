use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn fruits_background(attrs: Attrs) -> Markup {
    html! {
        div.fruits-background bq-setup aria-hidden="true" (..attrs) {
            canvas bq-ref="canvas" {}
        }
    }
}
