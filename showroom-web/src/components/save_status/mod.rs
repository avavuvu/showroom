use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::Button;

#[component]
pub fn save_status() -> Markup {
    html! {
        div.save-status-group {
            output.save-status bq-ref="status" data-state="saved" { "Saved" }
            span.save-actions bq-ref="save-actions" hidden {
                Button .small .secondary bq-ref="reload" { "Reload" }
                Button .small .secondary bq-ref="overwrite" { "Keep my version" }
            }
        }
    }
}
