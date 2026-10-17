use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::Button;

#[component]
pub fn save_status() -> Markup {
    html! {
        div.save-status-group {
            span.save-status bq-ref="status" data-state="saved" { "Saved" }
            span.sr-only role="status" aria-live="polite" bq-ref="save-announce" {}
            span.sr-only role="alert" aria-live="assertive" bq-ref="save-alert" {}
            span.save-actions bq-ref="save-actions" hidden {
                Button .small .secondary bq-on:click="reload" { "Reload" }
                Button .small .secondary bq-on:click="overwrite" { "Keep my version" }
            }
        }
    }
}
