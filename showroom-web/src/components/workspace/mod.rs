use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn workspace(left: Markup, right: Markup, notice: Option<&str>, attrs: Attrs, children: Markup) -> Markup {
    html! {
        div.workspace (..attrs) {
            @if let Some(notice) = notice {
                div.workspace-notice { (notice) }
            }
            header.workspace-header {
                div.left { (left) }
                div.right { (right) }
            }
            (children)
        }
    }
}
