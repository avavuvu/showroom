mod link_bar;
mod save_status;
mod toolbar;

use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

use crate::components::Button;
use link_bar::LinkBar;
use save_status::SaveStatus;
use toolbar::Toolbar;

setup!(NewsletterEditor);

#[component]
pub fn editor_page(
    props: &str,
    title: &str,
    #[builder(required)] subtitle: Option<&str>,
    back: &str,
    next: &str,
    #[builder(default)] sent: bool,
) -> Markup {
    html! {
        div.edit-view
            bq-setup=(NewsletterEditor)
            bq-on:keydown|mod|s|prevent|window="save"
            bq-on:beforeunload|window="guard"
            data-props=(props) {
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

            Editor(title = title, subtitle = subtitle);
        }
    }
}

#[component]
fn editor(title: &str, #[builder(required)] subtitle: Option<&str>) -> Markup {
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
                    bq-ref="subtitle"
                    bq-on:input="mark-dirty"
                    bq-on:keydown|enter|down|prevent="to-body";
            }

            div.editor-body {
                div.editor-surface.prose bq-ref="surface" {}
            }

            input type="file" accept="image/png,image/jpeg,image/gif,image/webp" multiple hidden bq-ref="file-input" bq-on:change="files-picked";

            template bq-ref="image-frame" {
                div.image-frame {
                    div.image-actions {
                        button type="button" data-action="replace" { "Replace" }
                        button type="button" data-action="width" {
                            span.when-normal { "Full width" }
                            span.when-full { "Normal width" }
                        }
                        button type="button" data-action="remove" { "Remove" }
                    }
                    span.upload-label role="status" { "Uploading…" }
                }
            }

            template bq-ref="upload-placeholder" {
                figure.image-block.is-uploading contenteditable="false" {
                    div.image-frame {
                        span.upload-label role="status" { "Uploading…" }
                    }
                }
            }
        }
    }
}
