use boutique::html;
use bq_components::component;
use maud::Markup;

#[component]
pub fn image_uploading() -> Markup {
    html! {
        figure.image-block.is-uploading contenteditable="false" {
            div.image-frame {
                img alt="";
                span.upload-label role="status" { "Uploading\u{2026}" }
            }
        }
    }
}
