use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn ascii_background(webm: &str, mp4: &str, attrs: Attrs) -> Markup {
    html! {
        div.ascii-background bq-setup (..attrs) {
            video bq-ref="video" autoplay muted loop playsinline {
                source src=(webm) type="video/webm";
                source src=(mp4) type="video/mp4";
            }
            canvas bq-ref="canvas" {}
        }
    }
}
