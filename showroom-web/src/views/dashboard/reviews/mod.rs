use boutique::html;
use maud::{Markup, PreEscaped};

use crate::components::{Button, CopyField, LocalTime, ReviewRequest, Workspace};
use crate::document::{Block, Document, merge::{Placement, Planned}};
use crate::models::{newsletter, review_link::{self, Progress}};
use crate::renderer::{diff, html::render};
use crate::views::context::PageContext;
use crate::views::layouts::{base, page};

#[derive(Default)]
pub struct NewReviewForm {
    pub section_id: Option<String>,
    pub note: Option<String>,
    pub names: Vec<String>,
    pub error: Option<String>,
}

pub struct PersonRow {
    pub link: review_link::Model,
    pub url: String,
    pub results_url: String,
    pub delete_url: String,
}

pub struct ReviewGroup {
    pub label: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub people: Vec<PersonRow>,
}

pub struct ResultsView<'a> {
    pub reviews_url: String,
    pub editor_url: String,
    pub apply_url: String,
    pub restore_url: String,
    pub proposed: &'a [Block],
    pub current: &'a [Block],
    pub planned: &'a [Planned],
    pub deleted: bool,
    pub newsletter_revision: i32,
    pub link_revision: i32,
    pub applied: Option<usize>,
    pub stale: bool,
    pub link: &'a review_link::Model,
}

fn date(value: &chrono::DateTime<chrono::FixedOffset>) -> String {
    value.format("%B %-d").to_string()
}

fn workspace_page(ctx: &PageContext, name: &str, left: Markup, right: Markup, body: Markup) -> Markup {
    base(
        &page(name),
        html! {
            style { (PreEscaped(ctx.publication().theme().css())) }
            Workspace(left = left, right = right) .review-workspace {
                main.workspace-body { (body) }
            }
        },
    )
}

fn progress(link: &review_link::Model) -> Markup {
    let (class, text) = match link.progress() {
        Progress::NotOpened => ("is-idle", "Not opened".to_string()),
        Progress::Opened => ("is-idle", "Opened".to_string()),
        Progress::Editing => ("is-active", "Editing".to_string()),
        Progress::Sent => ("is-done", format!("Sent back {}", link.sent_at.as_ref().map(date).unwrap_or_default())),
        Progress::EditedSinceSent => ("is-done", "Sent back, edited since".to_string()),
    };
    html! {
        span.review-progress.(class) { (text) }
        @if link.applied_at.is_some() {
            span.review-progress.is-applied { "Applied" }
        }
    }
}

pub fn new_review(ctx: &PageContext, newsletter: &newsletter::Model, snapshot: &Document, form: &NewReviewForm) -> Markup {
    let editor = format!("/{}/edit/{}", ctx.publication().slug, newsletter.id);

    workspace_page(
        ctx,
        "Send for suggestions",
        html! { Button(href = &editor) .secondary { "Back" } },
        html! { Button(submit = true) .primary form="review-request" { "Create links" } },
        html! {
            ReviewRequest(
                action = &format!("{editor}/review"),
                section_id = [form.section_id.as_deref()],
                names = &form.names,
                note = [form.note.as_deref()],
                error = [form.error.as_deref()],
            );
            div.prose.review-snapshot {
                div.prose-body.flow {
                    @if form.section_id.is_none() {
                        h1 { (newsletter.display_title()) }
                    }
                    (render(snapshot))
                }
            }
        },
    )
}

pub fn index(ctx: &PageContext, groups: &[ReviewGroup], editor: &str) -> Markup {
    workspace_page(
        ctx,
        "Suggestions",
        html! { Button(href = editor) .secondary { "Back" } },
        html! {},
        html! {
            div.review-start {
                Button(href = &format!("{editor}/review")) .primary { "Send Newsletter for Suggestions" }
            }
            @for group in groups {
                section.review-group {
                    header {
                        h2 { (group.label) }
                        LocalTime(&group.created_at);
                    }
                    ul.review-people {
                        @for person in &group.people {
                            li {
                                div.review-person-head {
                                    span.review-person { (person.link.reviewer_name) }
                                    (progress(&person.link))
                                    @if person.link.proposed.is_some() {
                                        a.review-open href=(person.results_url) { "View" }
                                    }
                                    form.review-delete method="post" action=(person.delete_url) {
                                        button type="submit" aria-label=(format!("Delete {}\u{2019}s link", person.link.reviewer_name)) title="Delete link" { "\u{2715}" }
                                    }
                                }
                                CopyField(&person.url, label = &format!("Link for {}", person.link.reviewer_name));
                            }
                        }
                    }
                }
            }
        },
    )
}

fn change(view: &ResultsView<'_>, index: usize, item: &Planned) -> Markup {
    let proposed = item.replacement(view.proposed);
    if item.placement == Placement::Applied {
        return diff::blocks(proposed);
    }
    let is_conflict = item.placement == Placement::Conflict;
    let can_tick = !view.deleted && item.can_apply();
    html! {
        div.review-change.is-conflict[is_conflict] {
            @if can_tick {
                label.review-tick title=(if is_conflict { "You also edited this. Apply to replace yours." } else { "Apply" }) {
                    input type="checkbox" name="change" value=(index) form="review-apply" checked[!is_conflict];
                }
            }
            (diff::change(&view.current[item.current.clone()], proposed))
        }
    }
}

fn revision_fields(view: &ResultsView<'_>) -> Markup {
    html! {
        input type="hidden" name="revision" value=(view.newsletter_revision);
        input type="hidden" name="link_revision" value=(view.link_revision);
    }
}

pub fn results(ctx: &PageContext, view: &ResultsView<'_>) -> Markup {
    let name = &view.link.reviewer_name;
    let applicable = !view.deleted && view.planned.iter().any(Planned::can_apply);
    let mut parts = Vec::new();
    let mut cursor = 0;
    for (index, item) in view.planned.iter().enumerate() {
        parts.push(diff::blocks(&view.proposed[cursor..item.hunk.proposed.start]));
        parts.push(change(view, index, item));
        cursor = item.hunk.proposed.end;
    }
    parts.push(diff::blocks(&view.proposed[cursor..]));

    workspace_page(
        ctx,
        &format!("{name}\u{2019}s suggestions"),
        html! {
            Button(href = &view.reviews_url) .secondary { "Back" }
            span.workspace-label { (name) }
        },
        html! {
            @if view.deleted && !view.proposed.is_empty() {
                form method="post" action=(view.restore_url) {
                    (revision_fields(view))
                    Button(submit = true) .primary { "Add back to newsletter" }
                }
            } @else if applicable {
                form #review-apply method="post" action=(view.apply_url) {
                    (revision_fields(view))
                    Button(submit = true) .primary { "Apply" }
                }
            }
        },
        html! {
            @match view.applied {
                Some(0) => { p.review-notice { "Nothing selected." } }
                Some(_) => { p.review-notice { "Applied. " a href=(view.editor_url) { "Open the editor" } } }
                None => {}
            }
            @if view.stale {
                p.review-notice.is-warning { "The newsletter changed. Check again." }
            }
            @if view.deleted {
                p.review-notice.is-warning { "You deleted this section." }
            }
            @if let Some(reply) = &view.link.reply {
                blockquote.review-reply { (reply) }
            }
            @if view.planned.is_empty() && !view.deleted {
                p.review-empty { "No changes." }
            } @else {
                div.prose.review-diff {
                    div.prose-body.flow {
                        @for part in &parts { (part) }
                    }
                }
            }
        },
    )
}
