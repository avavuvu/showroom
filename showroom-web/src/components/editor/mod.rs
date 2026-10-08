mod toolbar;

use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

use toolbar::{LinkBar, Toolbar};

setup!(pub NewsletterEditor);

#[component]
pub fn editor(title: &str, #[builder(required)] subtitle: Option<&str>) -> Markup {
    html! {
        div.newsletter-editor {
            div.editor-toolbar {
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
                    bq-ref="title";
                input.editor-subtitle
                    type="text"
                    name="subtitle"
                    value=(subtitle.unwrap_or(""))
                    placeholder="Subtitle (optional)"
                    aria-label="Subtitle"
                    autocomplete="off"
                    bq-ref="subtitle";
            }

            div.editor-body bq-ref="body" {
                div.editor-surface.prose bq-ref="surface" {}
            }

            input type="file" accept="image/png,image/jpeg,image/gif,image/webp" multiple hidden bq-ref="file-input";

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
