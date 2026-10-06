use bq_components::{component, setup};
use maud::{Markup, html};

setup!(SaveStatus);

#[component]
pub fn save_status() -> Markup {
    html! {
        output.save-status aria-live="polite" bq-setup=(SaveStatus) { "Saved" }
    }
}
