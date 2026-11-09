mod editor;

use boutique::{component, html};
use maud::Markup;

use crate::{
    components::{Button, LocalTime},
    services::comments::{Badge, Commenter, Field, Login, MAX_BODY, MAX_NAME, Rejection, Shown, content::Invalid},
};

use editor::CommentEditor;

pub enum Outcome<'a> {
    Blank,
    Posted { id: String },
    AwaitingConfirmation,
    AwaitingSubscription,
    Rejected { reason: Rejection, draft: Draft<'a> },
}

#[derive(Default)]
pub struct Draft<'a> {
    pub body: &'a str,
    pub content: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}

pub struct Publishable {
    pub id: String,
    pub subscribes: bool,
}

pub struct Discussion<'a> {
    pub newsletter_url: String,
    pub publication_name: &'a str,
    pub comments: Vec<Shown>,
    pub is_owner: bool,
    pub commenter: &'a Commenter,
    pub publishable: Option<Publishable>,
}

fn badge(badge: Option<Badge>) -> Markup {
    html! {
        @match badge {
            Some(Badge::Author) => span.comment-badge.author { "Author" },
            Some(Badge::Member) => span.comment-badge { "Showroom" },
            None => {},
        }
    }
}

#[component]
pub fn comment_item(
    #[positional] comment: &Shown,
    #[positional] newsletter_url: &str,
    is_owner: bool,
    #[default] is_new: bool,
    publishable: Option<&Publishable>,
) -> Markup {
    let item_url = format!("{newsletter_url}/comments/{}", comment.id);
    let publish_url = format!("{item_url}/publish");

    html! {
        @if comment.is_awaiting {
            li.comment.awaiting
                id={ "comment-" (comment.id) }
                hx-get=(item_url)
                hx-trigger="focus from:window, visibilitychange from:document"
                hx-target="this"
                hx-swap="outerHTML"
            {
                header.comment-meta {
                    span.comment-name { (comment.name) }
                    span.comment-badge { "PENDING CONFIRMATION" }
                }
                div.comment-body.comment-rich { (comment.content.html()) }
                @if let Some(publishable) = publishable {
                    form.comment-publish method="POST" action=(publish_url) hx-post=(publish_url) hx-target="#comments" hx-swap="outerHTML" {
                        Button(submit = true) .primary {
                            @if publishable.subscribes { "Subscribe and publish" } @else { "Publish" }
                        }
                    }
                } @else {
                    p.comment-awaiting-note { "Only you can see this. Your comment will appear once you confirm it." }
                }
            }

        } @else {
            li.comment.is-new[is_new] id={ "comment-" (comment.id) } {
                header.comment-meta {
                    @if let Some(href) = &comment.href {
                        a.comment-name href=(href) { (comment.name) }
                    } @else {
                        span.comment-name { (comment.name) }
                    }
                    (badge(comment.badge))
                    LocalTime(&comment.created_at);
                    @if is_owner {
                        button.comment-delete
                            type="button"
                            hx-delete=(item_url)
                            hx-target="#comments"
                            hx-swap="outerHTML"
                            hx-confirm="Delete this comment?"
                        {
                            "Delete"
                        }
                    }
                }
                div.comment-body.comment-rich { (comment.content.html()) }
            }
        }
    }
}

fn notice(text: Markup) -> Markup {
    html! {
        p.comment-notice role="status" { (text) }
    }
}

fn rejection_message(reason: Rejection) -> &'static str {
    match reason {
        Rejection::TooMany => "You're commenting too quickly. Please try again later.",
        Rejection::Content(Invalid::Empty) => "Write something first.",
        Rejection::Content(Invalid::TooLong) => "Comments can be up to 5,000 characters.",
        Rejection::Content(Invalid::Unsupported) => "That formatting isn't supported in comments.",
        Rejection::Invalid(Field::Name) => "Names can be up to 80 characters.",
        Rejection::Invalid(Field::Email) => "Enter a valid email address.",
        Rejection::Expired => "This comment can no longer be published. Please post it again.",
        Rejection::EmailNeeded => "Enter your email to comment.",
        Rejection::EmailFailed => "We couldn't send the email. Please try again.",
    }
}

fn outcome_notice(discussion: &Discussion, outcome: &Outcome) -> Markup {
    html! {
        @match outcome {
            Outcome::Blank => {},
            Outcome::Posted { .. } => {},
            Outcome::AwaitingConfirmation => (notice(html! { "Check your email: we've sent you a link to publish your comment." })),
            Outcome::AwaitingSubscription => (notice(html! {
                "Your comment will appear once you have subscribed to " (discussion.publication_name) "! Check your email for the confirmation link."
            })),
            Outcome::Rejected { reason, .. } => p.error role="alert" { (rejection_message(*reason)) },
        }
    }
}

fn sign_out_button(newsletter_url: &str, label: &str) -> Markup {
    let action = format!("{newsletter_url}/comments/sign-out");
    html! {
        button.comment-link
            type="submit"
            formaction=(action)
            formnovalidate
            hx-post=(action)
            hx-target="#comments"
            hx-swap="outerHTML"
        {
            (label)
        }
    }
}

fn identity(discussion: &Discussion, draft: &Draft) -> Markup {
    html! {
        @match discussion.commenter {
            Commenter::Member(member, login) => {
                span.comment-name { (member.name) }
                (badge(Some(if discussion.is_owner { Badge::Author } else { Badge::Member })))
                @if *login == Login::Link {
                    (sign_out_button(&discussion.newsletter_url, "Sign out"))
                }
            },
            Commenter::Subscriber(subscriber) => {
                span { "Commenting as " strong { (subscriber.name.as_deref().unwrap_or("Anonymous")) } }
                (sign_out_button(&discussion.newsletter_url, "Not you?"))
            },
            Commenter::Reader => {
                input.comment-inline #comment-name
                    type="text"
                    name="name"
                    value=(draft.name)
                    placeholder="Your name (optional)"
                    aria-label="Name"
                    autocomplete="name"
                    maxlength=(MAX_NAME);
                input.comment-inline #comment-email
                    type="email"
                    name="email"
                    value=(draft.email)
                    placeholder="you@example.com"
                    aria-label="Email"
                    autocomplete="email"
                    required;
            },
        }
    }
}

#[component]
pub fn comments(#[positional] discussion: &Discussion<'_>, #[positional] outcome: &Outcome<'_>) -> Markup {
    let action = format!("{}/comments", discussion.newsletter_url);
    let draft = match outcome {
        Outcome::Rejected { draft, .. } => draft,
        _ => &Draft::default(),
    };
    let published = discussion.comments.iter().filter(|comment| !comment.is_awaiting).count();
    let posted = match outcome {
        Outcome::Posted { id } => Some(id.as_str()),
        _ => None,
    };

    html! {
        section.comments #comments {
            h2 {
                "Comments"
                @if published > 0 {
                    " " span.comment-count { (published) }
                }
            }

            form.comment-form method="POST" action=(action) hx-post=(action) hx-target="#comments" hx-swap="outerHTML" {
                div.comment-compose {
                    CommentEditor(draft.body, draft.content, max = MAX_BODY);
                    div.comment-compose-bar {
                        div.comment-identity { (identity(discussion, draft)) }
                        Button(submit = true) .primary { "Comment" }
                    }
                }
                (outcome_notice(discussion, outcome))
                div.comment-website aria-hidden="true" {
                    label for="comment-website" { "Website" }
                    input #comment-website type="text" name="website" tabindex="-1" autocomplete="off";
                }
            }

            @if !discussion.comments.is_empty() {
                ol.comment-list {
                    @for comment in &discussion.comments {
                        CommentItem(comment, &discussion.newsletter_url, is_owner = discussion.is_owner, is_new = posted == Some(comment.id.as_str()), publishable = [discussion.publishable.as_ref().filter(|publishable| publishable.id == comment.id)]);
                    }
                }
            }
        }
    }
}
