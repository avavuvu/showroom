mod link_bar;
mod save_status;
mod toolbar;

use boutique::html;
use bq_components::component;
use maud::Markup;
use serde::Serialize;

use crate::blocks::{
    image::{ImageEditor, ImageUploading},
    music::{MusicEditor, MusicLookup},
};
use crate::components::Button;
use crate::document::Document;
use link_bar::LinkBar;
use save_status::SaveStatus;
use toolbar::Toolbar;

#[boutique::ts]
#[derive(Serialize)]
pub struct EditorProps<'a> {
    pub id: &'a str,
    pub revision: i32,
    #[ts(type = "unknown")]
    pub content: Document,
}

#[component(setup = "newsletter-editor")]
pub fn editor_page(
    props: &EditorProps<'_>,
    title: &str,
    subtitle: Option<&str>,
    back: &str,
    next: &str,
    sent: bool,
) -> Markup {
    html! {
        div.edit-view
            bq-setup
            bq-on:keydown|mod|s|prevent|window="save"
            bq-on:beforeunload|window="guard"
            data-props=(serde_json::to_string(props).unwrap_or_default()) {
            @if sent {
                div.marquee {
                    "This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes."
                }
            }

            header bq-ref="header" {
                div.left {
                    Button(href = back) .secondary bq-on:click|left|exact="leave" { "Back" }
                    SaveStatus;
                }

                div {
                    Button(href = next) .primary bq-on:click|left|exact="leave" {
                        @if sent { "View" } @else { "Publish" }
                    }
                }
            }

            Editor(title = title, subtitle = [subtitle]);
        }
    }
}

#[component]
fn editor(title: &str, subtitle: Option<&str>) -> Markup {
    html! {
        div.newsletter-editor {
            div.editor-toolbar bq-ref="toolbar" {
                Toolbar;
                LinkBar;
            }

            div.editor-meta {
                input.editor-title
                    type="text"
                    name="title"
                    value=(title)
                    placeholder="Title"
                    aria-label="Title"
                    autocomplete="off"
                    autocapitalize="sentences"
                    enterkeyhint="next"
                    bq-ref="title"
                    bq-on:input="mark-dirty"
                    bq-on:keydown|enter|prevent="to-subtitle";
                input.editor-subtitle
                    type="text"
                    name="subtitle"
                    value=(subtitle.unwrap_or(""))
                    placeholder="Subtitle (optional)"
                    aria-label="Subtitle"
                    autocomplete="off"
                    autocapitalize="sentences"
                    enterkeyhint="next"
                    bq-ref="subtitle"
                    bq-on:input="mark-dirty"
                    bq-on:keydown|enter|down|prevent="to-body";
            }

            div.editor-body {
                div.editor-surface.prose bq-ref="surface" {}
            }

            input type="file" accept="image/png,image/jpeg,image/gif,image/webp" multiple hidden bq-ref="file-input" bq-on:change="files-picked";

            template bq-ref="image-frame" {
                ImageEditor;
            }

            template bq-ref="upload-placeholder" {
                ImageUploading;
            }

            template bq-ref="music-frame" {
                MusicEditor;
            }

            template bq-ref="music-lookup" {
                MusicLookup;
            }
        }
    }
}
