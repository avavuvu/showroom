mod link_bar;
mod save_status;
mod section_menu;
mod toolbar;

use boutique::html;
use bq_components::component;
use maud::Markup;
use serde::Serialize;

use crate::blocks::{
    image::{ImageEditor, ImageUploading},
    music::{MusicEditor, MusicLookup},
    youtube::YoutubeEditor,
    boxout::BoxoutEditor,
};
use crate::components::{Button, LocalTime, Workspace};
use crate::document::Document;
use crate::models::review::ReviewSummary;
use link_bar::LinkBar;
use save_status::SaveStatus;
use section_menu::SectionMenu;
use toolbar::Toolbar;

#[boutique::ts]
#[derive(Serialize)]
pub struct EditorProps<'a> {
    pub id: &'a str,
    pub slug: &'a str,
    pub revision: i32,
    #[ts(type = "unknown")]
    pub content: Document,
    pub reviews: Vec<ReviewSummary>,
}

#[boutique::ts]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewProps<'a> {
    pub url: &'a str,
    pub revision: i32,
    pub section_id: Option<&'a str>,
    #[ts(type = "unknown")]
    pub content: Document,
    #[ts(type = "unknown")]
    pub base: Document,
    pub sent: bool,
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
    let returned: usize = props.reviews.iter().map(|review| review.returned).sum();
    html! {
        Workspace(
            notice = [sent.then_some("This newsletter has been sent. Changes made now will update online, but not in your subscribers' inboxes.")],
            left = html! {
                Button(href = back) .secondary bq-on:click|left|exact="leave" { "Back" }
                SaveStatus;
            },
            right = html! {
                Button(href = &format!("/{}/edit/{}/reviews", props.slug, props.id)) .secondary .reviews-button bq-on:click|left|exact="leave" {
                    "Suggestions"
                    @if returned > 0 {
                        span.reviews-count aria-label=(format!("{returned} sent back")) { (returned) }
                    }
                }
                Button(href = next) .primary bq-on:click|left|exact="leave" {
                    @if sent { "View" } @else { "Publish" }
                }
            },
        )
            .edit-view
            bq-setup
            bq-on:keydown|mod|s|prevent|window="save"
            bq-on:beforeunload|window="guard"
            bq-on:pageshow|window="restore"
            data-props=(serde_json::to_string(props).unwrap_or_default())
        {
            Editor(title = title, subtitle = [subtitle]);
        }
    }
}

fn toolbar_bar(media: bool) -> Markup {
    html! {
        div.editor-toolbar bq-ref="toolbar" {
            Toolbar(media = media);
            LinkBar;
        }
    }
}

fn block_frames() -> Markup {
    html! {
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

        template bq-ref="youtube-frame" {
            YoutubeEditor;
        }

        template bq-ref="boxout-frame" {
            BoxoutEditor;
        }
    }
}

#[component]
fn editor(title: &str, subtitle: Option<&str>) -> Markup {
    html! {
        div.newsletter-editor {
            (toolbar_bar(true))

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
                div.editor-surface.prose bq-ref="surface" {
                    SectionMenu;
                }
            }

            input type="file" accept="image/png,image/jpeg,image/gif,image/webp" multiple hidden bq-ref="file-input" bq-on:change="files-picked";

            (block_frames())
        }
    }
}

#[component(setup = "review-editor")]
pub fn review_page(
    props: &ReviewProps<'_>,
    publication: &str,
    newsletter: &str,
    note: Option<&str>,
    reply: Option<&str>,
    sent_at: Option<&chrono::DateTime<chrono::FixedOffset>>,
) -> Markup {
    html! {
        Workspace(
            left = html! {
                span.workspace-label { (publication) }
                SaveStatus;
            },
            right = html! {
                p.workspace-warning role="alert" bq-ref="send-error" {}
                span.workspace-label role="status" bq-ref="sent-status" {
                    @if let Some(sent_at) = sent_at { "Sent " LocalTime(sent_at); }
                }
                Button .primary bq-ref="send" bq-on:click="send" {
                    @if props.sent { "Send update" } @else { "Send suggestions" }
                }
            },
        )
            .review-view
            bq-setup
            bq-on:keydown|mod|s|prevent|window="save"
            bq-on:beforeunload|window="guard"
            data-props=(serde_json::to_string(props).unwrap_or_default())
        {
            div.newsletter-editor.is-review {
                (toolbar_bar(false))

                @if props.section_id.is_none() || note.is_some() {
                    div.editor-meta.prose {
                        @if let Some(note) = note {
                            blockquote.review-note { (note) }
                        }
                        @if props.section_id.is_none() {
                            h1.review-title { (newsletter) }
                        }
                    }
                }

                div.editor-body {
                    div.editor-surface.prose bq-ref="surface" {}
                }

                div.review-reply-box {
                    textarea.review-reply name="reply" rows="3" maxlength="2000" placeholder=(format!("Message for {publication} (optional)")) aria-label="Message" bq-ref="reply" bq-on:input="reply-edited" { (reply.unwrap_or("")) }
                }

                (block_frames())
            }
        }
    }
}
