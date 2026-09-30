use bq_components::{component, setup};
use maud::{Markup, html};

setup!(SaveStatus);

#[component]
pub fn save_status() -> Markup {
    html! {
        sr-save-status.save-status bq-setup=(SaveStatus) { "Saved" }
    }
}
