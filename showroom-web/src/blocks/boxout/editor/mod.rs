use boutique::html;
use bq_components::component;
use maud::Markup;

#[component(setup = "boxout-editor")]
pub fn boxout_editor() -> Markup {
    html! {
        aside.boxout.boxout-editor bq-setup {
            div.boxout-controls contenteditable="false" {
                div.boxout-group role="group" aria-label="Boxout style" {
                    button type="button" data-style="bordered" bq-ref="style" bq-on:click="set-style" { "Bordered" }
                    button type="button" data-style="brand" bq-ref="style" bq-on:click="set-style" { "Brand" }
                    button type="button" data-style="trim" bq-ref="style" bq-on:click="set-style" { "Trim" }
                }
                div.boxout-group {
                    button type="button" bq-on:click="unwrap" { "Unwrap" }
                    button type="button" bq-on:click="remove" { "Delete" }
                }
            }
            span.boxout-handle.is-start contenteditable="false" aria-hidden="true" data-edge="start" bq-ref="handle" bq-on:pointerdown="drag" {}
            div.boxout-content bq-ref="content" {}
            span.boxout-handle.is-end contenteditable="false" aria-hidden="true" data-edge="end" bq-ref="handle" bq-on:pointerdown="drag" {}
        }
    }
}
