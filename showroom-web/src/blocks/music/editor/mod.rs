use boutique::html;
use bq_components::component;
use maud::Markup;

use super::MusicService;

#[component(setup = "music-editor")]
pub fn music_editor() -> Markup {
    html! {
        figure.music-card.music-editor contenteditable="false" bq-setup {
            div.music-artwork {
                img alt="" draggable="false" hidden bq-ref="artwork";
                button.music-cover type="button" bq-on:click="pick-cover" {
                    span.when-empty { "Add cover" }
                    span.when-set { "Replace cover" }
                }
            }
            div.music-details {
                p.music-kind hidden bq-ref="kind" { "Album" }
                input.music-title-input
                    type="text"
                    placeholder="Title"
                    aria-label="Title"
                    autocomplete="off"
                    spellcheck="false"
                    bq-ref="title"
                    bq-on:input="edit-title";
                input.music-artist-input
                    type="text"
                    placeholder="Artist"
                    aria-label="Artist"
                    autocomplete="off"
                    spellcheck="false"
                    bq-ref="artist"
                    bq-on:input="edit-artist";
                ul.music-links {
                    @for service in MusicService::ALL {
                        li hidden data-service=(service.as_str()) bq-ref="link" {
                            a target="_blank" rel="noopener noreferrer" { (service.label()) }
                            button
                                type="button"
                                aria-label={ "Remove the " (service.label()) " link" }
                                title={ "Remove the " (service.label()) " link" }
                                bq-on:click="unlink"
                            {
                                "\u{00d7}"
                            }
                        }
                    }
                }
                input.music-add
                    type="text"
                    inputmode="url"
                    placeholder="Paste a link from another service"
                    aria-label="Add a link from another service"
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                    enterkeyhint="done"
                    bq-ref="add"
                    bq-on:keydown|enter|prevent="add-link"
                    bq-on:keydown|escape|prevent="clear-add";
                span.music-status role="status" bq-ref="status" {}
            }
            div.music-actions {
                button type="button" bq-on:click="remove" { "Remove" }
            }
        }
    }
}
