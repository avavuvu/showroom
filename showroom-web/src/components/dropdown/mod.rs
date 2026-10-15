use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn dropdown(attrs: Attrs, children: Markup) -> Markup {
    html! {
        details
            bq-setup
            bq-on:keydown|escape="close"
            bq-on:pointerdown|document="outside"
            (..attrs)
        {
            (children)
        }
    }
}
