use boutique::html;
use bq_components::component;
use maud::Markup;

#[component]
pub fn music_lookup() -> Markup {
    html! {
        figure.music-card.music-lookup contenteditable="false" {
            span.music-lookup-message role="status" {}
        }
    }
}
