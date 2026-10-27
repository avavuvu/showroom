use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::Button;

#[component(setup = "copy-field")]
pub fn copy_field(#[positional] value: &str, label: &str) -> Markup {
    html! {
        div.copy-field bq-setup {
            input.copy-field-input type="text" readonly value=(value) aria-label=(label) bq-ref="input" bq-on:focus="select";
            Button .secondary .small bq-ref="button" bq-on:click="copy" { "Copy" }
        }
    }
}
