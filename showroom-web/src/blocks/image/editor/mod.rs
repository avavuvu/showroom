use boutique::html;
use bq_components::component;
use maud::Markup;

#[component(setup = "image-editor")]
pub fn image_editor() -> Markup {
    html! {
        div.image-frame bq-setup {
            img alt="" draggable="false" bq-ref="image";
            div.image-actions {
                button.image-replace type="button" bq-on:click="replace" { "Replace" }
                button.image-width-toggle type="button" bq-on:click="toggle-width" {
                    span.when-normal { "Full width" }
                    span.when-full { "Normal width" }
                }
                button type="button" bq-on:click="remove" { "Remove" }
            }
            span.upload-label role="status" { "Uploading\u{2026}" }
        }
    }
}
