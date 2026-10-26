use boutique::html;
use bq_components::component;
use maud::Markup;

use super::{WATCH_LABEL, play_badge};

#[component(setup = "youtube-editor")]
pub fn youtube_editor() -> Markup {
    html! {
        figure.media-card.youtube-card.youtube-editor contenteditable="false" bq-setup {
            div.youtube-frame {
                div.youtube-poster {
                    img alt="" draggable="false" bq-ref="thumbnail";
                    (play_badge())
                }
            }
            div.media-card-details bq-ref="details" {
                p.media-card-title bq-ref="title" {}
                p.media-card-byline bq-ref="channel" {}
                span.youtube-status role="status" bq-ref="status" {}
            }
            div.youtube-actions {
                a target="_blank" rel="noopener noreferrer" bq-ref="open" { (WATCH_LABEL) }
                button type="button" bq-on:click="remove" { "Remove" }
            }
        }
    }
}
