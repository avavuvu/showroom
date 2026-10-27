use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::Button;

fn name_row(value: &str) -> Markup {
    html! {
        div.reviewer-row {
            input type="text" name="reviewer" value=(value) maxlength="60" autocomplete="off" placeholder="Name" aria-label="Reviewer name";
            button.reviewer-remove type="button" aria-label="Remove" title="Remove" bq-on:click="review-request.remove-row" { "\u{2715}" }
        }
    }
}

#[component(setup = "review-request")]
pub fn review_request(
    action: &str,
    section_id: Option<&str>,
    names: &[String],
    note: Option<&str>,
    error: Option<&str>,
) -> Markup {
    html! {
        form.review-request #review-request method="post" action=(action) bq-setup {
            @if let Some(section_id) = section_id {
                input type="hidden" name="section" value=(section_id);
            }
            div.reviewer-rows bq-ref="rows" {
                @if names.is_empty() {
                    (name_row(""))
                }
                @for name in names {
                    (name_row(name))
                }
            }
            template bq-ref="row-template" { (name_row("")) }
            Button .secondary .small bq-on:click="add-row" { "Add reviewer" }
            textarea name="note" rows="3" maxlength="1000" placeholder="Note (optional)" aria-label="Note" { (note.unwrap_or("")) }
            @if let Some(error) = error {
                p.error role="alert" { (error) }
            }
        }
    }
}
