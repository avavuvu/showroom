use std::collections::HashMap;

use axum::{
    Form,
    extract::{FromRequestParts, Path, Query, State},
    http::{HeaderMap, StatusCode, request::Parts},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use boutique::{AppError, AppResult, AuthenticatedUser, forms::blank_to_none, html, htmx};
use maud::Markup;
use sea_orm::DbErr;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    components::{CommentItem, Comments, comments::{Discussion, Draft, Outcome, Publishable}},
    handlers, mailer,
    models::{
        comment::{self, Status},
        newsletter::{self, Entity as Newsletter},
        publication, user,
    },
    services::{
        comments::{self, Author, Commenter, Content, MAX_NAME, Pending, Rejection, credential, lifecycle::{self, Verified}},
        publication::OwnedPublication,
        subdomain::CurrentPublication,
    },
    state::AppState,
    views::{
        self, PageContext, Viewer,
        comments::{Unavailable, email::CommentConfirmation},
    },
};

type MailResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

enum SubmitError {
    Rejected(Rejection),
    App(AppError),
}

impl From<Rejection> for SubmitError {
    fn from(rejection: Rejection) -> Self {
        SubmitError::Rejected(rejection)
    }
}

impl From<ValidationErrors> for SubmitError {
    fn from(errors: ValidationErrors) -> Self {
        SubmitError::Rejected(errors.into())
    }
}

impl From<DbErr> for SubmitError {
    fn from(error: DbErr) -> Self {
        SubmitError::App(error.into())
    }
}

#[derive(Deserialize, Validate)]
pub struct CommentForm {
    #[serde(default)]
    pub body: String,
    pub content: Option<String>,
    #[validate(length(max = MAX_NAME))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[serde(default)]
    pub website: String,
}

impl CommentForm {
    fn tidy(mut self) -> Self {
        self.body = self.body.trim().to_string();
        self.content = self.content.as_deref().and_then(blank_to_none);
        self.name = self.name.as_deref().and_then(blank_to_none);
        self.email = self.email.as_deref().and_then(blank_to_none).map(|email| email.to_lowercase());
        self
    }

    fn draft(&self) -> Draft<'_> {
        Draft { body: &self.body, content: self.content.as_deref().unwrap_or(""), name: self.name.as_deref().unwrap_or(""), email: self.email.as_deref().unwrap_or("") }
    }

    fn is_spam(&self) -> bool {
        !self.website.trim().is_empty()
    }
}

fn newsletter_url(state: &AppState, publication: &publication::Model, newsletter: &newsletter::Model) -> String {
    views::user::newsletter_url(&state.urls.publication(&publication.slug), newsletter)
}

pub struct Thread {
    state: AppState,
    publication: publication::Model,
    newsletter: newsletter::Model,
    commenter: Commenter,
    pending: Pending,
    confirming: Option<String>,
}

impl FromRequestParts<AppState> for Thread {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let CurrentPublication(publication) = CurrentPublication::from_request_parts(parts, state).await?;
        let Path(params) = Path::<HashMap<String, String>>::from_request_parts(parts, state).await.map_err(|_| AppError::NotFound)?;
        let slug = params.get("slug").ok_or(AppError::NotFound)?;
        let newsletter = Newsletter::find_published(&publication.id, slug).one(&state.db).await?.ok_or(AppError::NotFound)?;
        let viewer = <Viewer as FromRequestParts<AppState>>::from_request_parts(parts, state).await?;
        let jar = CookieJar::from_headers(&parts.headers);

        Thread::load(state, &jar, viewer.as_ref().map(|AuthenticatedUser(user)| user), publication, newsletter).await?.ok_or(AppError::NotFound)
    }
}

impl Thread {
    pub async fn load(
        state: &AppState,
        jar: &CookieJar,
        viewer: Option<&user::Model>,
        publication: publication::Model,
        newsletter: newsletter::Model,
    ) -> Result<Option<Self>, DbErr> {
        if !publication.comments_enabled {
            return Ok(None);
        }
        let commenter = credential::resolve(state, jar, viewer, &publication).await?;
        let pending = Pending::read(&state.signer, jar);
        let confirming = credential::confirming(&state.signer, jar);
        Ok(Some(Self { state: state.clone(), publication, newsletter, commenter, pending, confirming }))
    }

    fn url(&self) -> String {
        newsletter_url(&self.state, &self.publication, &self.newsletter)
    }

    async fn publishable(&self, comments: &[comment::Model]) -> Result<Option<Publishable>, DbErr> {
        let Some(comment) = comments.iter().find(|comment| self.confirming.as_ref() == Some(&comment.id) && lifecycle::is_awaiting(comment)) else {
            return Ok(None);
        };
        let email = comment.email.as_deref().unwrap_or_default();
        let subscribes = !comments::is_subscribed(&self.state.db, &self.publication.id, email).await?;
        Ok(Some(Publishable { id: comment.id.clone(), subscribes }))
    }

    pub async fn discussion(&self) -> Result<Discussion<'_>, DbErr> {
        let found = comments::for_newsletter(&self.state.db, &self.newsletter.id, &self.pending).await?;
        let publishable = self.publishable(&found).await?;
        Ok(Discussion {
            newsletter_url: self.url(),
            publication_name: &self.publication.name,
            comments: comments::shown(&self.state.db, &self.state.urls, &self.publication, found).await?,
            is_owner: self.commenter.is_owner(&self.publication),
            commenter: &self.commenter,
            publishable,
        })
    }

    async fn render(&self, outcome: &Outcome<'_>) -> AppResult<Markup> {
        let discussion = self.discussion().await?;
        Ok(html! { Comments(&discussion, outcome); })
    }

    async fn respond(&self, jar: CookieJar, headers: &HeaderMap, outcome: Outcome<'_>) -> AppResult<Response> {
        if htmx::is_htmx(headers) {
            Ok((jar, self.render(&outcome).await?).into_response())
        } else {
            let anchor = match &outcome {
                Outcome::Posted { id } => format!("comment-{id}"),
                _ => "comments".to_string(),
            };
            Ok((jar, Redirect::to(&format!("{}#{anchor}", self.url()))).into_response())
        }
    }

    async fn submit(&mut self, form: &CommentForm, headers: &HeaderMap, jar: CookieJar) -> Result<(Outcome<'static>, CookieJar), SubmitError> {
        form.validate()?;
        let content = Content::from_form(form.content.as_deref(), &form.body).map_err(Rejection::from)?;
        if form.is_spam() {
            return Ok((Outcome::AwaitingConfirmation, jar));
        }
        let email = match self.commenter {
            Commenter::Reader => Some(form.email.as_deref().ok_or(Rejection::EmailNeeded)?),
            _ => None,
        };
        comments::within_limits(&self.state.limits, headers, &self.publication.id, &self.commenter, email)?;

        let author = match (&self.commenter, email) {
            (Commenter::Member(member, _), _) => Author::Member(member),
            (Commenter::Subscriber(subscriber), _) => Author::Subscriber(subscriber),
            (Commenter::Reader, Some(email)) => return self.submit_claimed(email, form.name.as_deref(), &content, jar).await,
            (Commenter::Reader, None) => return Err(Rejection::EmailNeeded.into()),
        };
        let comment = lifecycle::create(&self.state.db, &self.newsletter.id, author, &content).await?;
        Ok((Outcome::Posted { id: comment.id }, jar))
    }

    async fn submit_claimed(&mut self, email: &str, name: Option<&str>, content: &Content, jar: CookieJar) -> Result<(Outcome<'static>, CookieJar), SubmitError> {
        let is_subscribed = comments::is_subscribed(&self.state.db, &self.publication.id, email).await?;
        let author = Author::Claimed { email, name };
        let comment = lifecycle::create(&self.state.db, &self.newsletter.id, author, content).await?;

        if let Err(error) = self.send_verification(email, &comment, !is_subscribed).await {
            tracing::error!(%error, publication = %self.publication.id, "sending a comment email failed");
            lifecycle::discard(&self.state.db, &comment.id).await?;
            return Err(Rejection::EmailFailed.into());
        }

        let jar = self.pending.remember(&self.state.signer, &self.state.urls, &comment.id, jar);
        let outcome = if is_subscribed { Outcome::AwaitingConfirmation } else { Outcome::AwaitingSubscription };
        Ok((outcome, jar))
    }

    async fn send_verification(&self, email: &str, comment: &comment::Model, subscribes: bool) -> MailResult {
        let AppState { ses, urls, signer, .. } = &self.state;
        let confirm_url = format!("{}/comments/confirm?token={}", urls.publication(&self.publication.slug), lifecycle::verify_token(signer, &comment.id));
        let content = Content::from_stored(&comment.content);
        let confirmation = CommentConfirmation {
            confirm_url: &confirm_url,
            publication_name: &self.publication.name,
            newsletter_title: self.newsletter.display_title(),
            comment: &content,
            subscribes,
        };
        mailer::send_comment_confirmation(ses, email, &confirmation, &self.publication, urls).await
    }
}

pub async fn post_comment(mut thread: Thread, jar: CookieJar, headers: HeaderMap, Form(form): Form<CommentForm>) -> AppResult<Response> {
    let form = form.tidy();
    let (outcome, jar) = match thread.submit(&form, &headers, jar.clone()).await {
        Ok(submitted) => submitted,
        Err(SubmitError::Rejected(reason)) => (Outcome::Rejected { reason, draft: form.draft() }, jar),
        Err(SubmitError::App(error)) => return Err(error),
    };
    thread.respond(jar, &headers, outcome).await
}

pub async fn show_comment(thread: Thread, Path((_, id)): Path<(String, String)>) -> AppResult<Markup> {
    let found = comments::in_newsletter(&thread.state.db, &thread.newsletter.id, &id).await?;
    let visible = found.filter(|comment| {
        comment.status == Status::Published || (lifecycle::is_awaiting(comment) && thread.pending.contains(&comment.id))
    });
    let Some(comment) = visible else {
        return Ok(html! {});
    };
    let publishable = thread.publishable(std::slice::from_ref(&comment)).await?;
    let shown = comments::shown(&thread.state.db, &thread.state.urls, &thread.publication, vec![comment]).await?;
    let is_owner = thread.commenter.is_owner(&thread.publication);
    let url = thread.url();
    Ok(html! {
        @for comment in &shown {
            CommentItem(comment, &url, is_owner = is_owner, publishable = [publishable.as_ref()]);
        }
    })
}

pub async fn publish_comment(mut thread: Thread, Path((_, id)): Path<(String, String)>, jar: CookieJar, headers: HeaderMap) -> AppResult<Response> {
    if thread.confirming.as_deref() != Some(id.as_str()) {
        return Err(AppError::Status(StatusCode::FORBIDDEN));
    }
    comments::in_newsletter(&thread.state.db, &thread.newsletter.id, &id).await?.ok_or(AppError::NotFound)?;
    let jar = credential::release_confirmation(jar);
    thread.confirming = None;

    let (outcome, jar) = match lifecycle::verify(&thread.state.db, &id).await? {
        Some(Verified::Published(comment)) => {
            let (jar, subscriber) = match comment.subscriber_token.as_deref() {
                Some(token) => credential::remember_subscriber(&thread.state, &thread.publication, token, jar).await?,
                None => (jar, None),
            };
            if let (Some(subscriber), Commenter::Reader) = (subscriber, &thread.commenter) {
                thread.commenter = Commenter::Subscriber(subscriber);
            }
            (Outcome::Posted { id: comment.id }, jar)
        }
        Some(Verified::Removed | Verified::Expired) | None => (Outcome::Rejected { reason: Rejection::Expired, draft: Draft::default() }, jar),
    };
    thread.respond(jar, &headers, outcome).await
}

pub async fn delete_comment(thread: Thread, Path((_, id)): Path<(String, String)>, jar: CookieJar, headers: HeaderMap) -> AppResult<Response> {
    if !thread.commenter.is_owner(&thread.publication) {
        return Err(AppError::Status(StatusCode::FORBIDDEN));
    }
    comments::in_newsletter(&thread.state.db, &thread.newsletter.id, &id).await?.ok_or(AppError::NotFound)?;
    lifecycle::remove(&thread.state.db, &id).await?;
    thread.respond(jar, &headers, Outcome::Blank).await
}

pub async fn sign_out(mut thread: Thread, jar: CookieJar, headers: HeaderMap) -> AppResult<Response> {
    let jar = credential::sign_out(&thread.state.urls, jar);
    thread.commenter = Commenter::Reader;
    thread.respond(jar, &headers, Outcome::Blank).await
}

pub async fn exchange_footer(state: &AppState, publication: &publication::Model, newsletter: &newsletter::Model, token: &str, jar: CookieJar) -> AppResult<Response> {
    let jar = credential::sign_in_subscriber(state, publication, &newsletter.id, token, jar).await?;
    Ok((jar, Redirect::to(&format!("{}#comments", newsletter_url(state, publication, newsletter)))).into_response())
}

#[derive(Deserialize)]
pub struct SessionQuery {
    pub token: String,
    pub to: Option<String>,
}

pub async fn start_session(State(state): State<AppState>, Query(query): Query<SessionQuery>, jar: CookieJar) -> AppResult<Response> {
    let location = query.to.as_deref().filter(|to| handlers::is_local_path(to)).unwrap_or("/");
    let (jar, _) = credential::sign_in_member(&state, &query.token, jar).await?;
    Ok((jar, Redirect::to(location)).into_response())
}

#[derive(Deserialize)]
pub struct VisitQuery {
    pub to: String,
}

pub async fn visit(State(state): State<AppState>, Query(query): Query<VisitQuery>, viewer: Viewer) -> AppResult<Redirect> {
    let target = url::Url::parse(&query.to).map_err(|_| AppError::NotFound)?;
    if !state.urls.is_publication(&target) {
        return Err(AppError::NotFound);
    }

    match viewer {
        Some(AuthenticatedUser(user)) if state.urls.needs_visit() => {
            let mut session = target.join("/comments/session").map_err(|_| AppError::NotFound)?;
            session
                .query_pairs_mut()
                .append_pair("token", &credential::sign_in_token(&state.signer, &user))
                .append_pair("to", &target[url::Position::BeforePath..]);
            Ok(Redirect::to(session.as_str()))
        }
        _ => Ok(Redirect::to(target.as_str())),
    }
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: String,
}

fn public_page(state: &AppState, viewer: Viewer, publication: &publication::Model) -> PageContext {
    PageContext::public(viewer, state.urls.clone()).with_publication(publication.clone())
}

pub async fn open_confirmation(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    viewer: Viewer,
    Query(query): Query<TokenQuery>,
    jar: CookieJar,
) -> AppResult<Response> {
    let ctx = public_page(&state, viewer, &publication);
    if !publication.comments_enabled {
        return Ok(views::comments::unavailable(&ctx, Unavailable::Disabled).into_response());
    }
    let found = match lifecycle::open_verify_token(&state.signer, &query.token) {
        Some(id) => comments::in_publication(&state.db, &publication.id, id).await?,
        None => None,
    };
    let Some((comment, newsletter)) = found else {
        return Ok(views::comments::unavailable(&ctx, Unavailable::Expired).into_response());
    };
    let location = format!("{}#comment-{}", newsletter_url(&state, &publication, &newsletter), comment.id);

    Ok(match comment.status {
        Status::Published => Redirect::to(&location).into_response(),
        Status::Removed => views::comments::unavailable(&ctx, Unavailable::Removed).into_response(),
        Status::Unverified if !lifecycle::is_awaiting(&comment) => views::comments::unavailable(&ctx, Unavailable::Expired).into_response(),
        Status::Unverified => {
            let mut pending = Pending::read(&state.signer, &jar);
            let jar = pending.remember(&state.signer, &state.urls, &comment.id, jar);
            let jar = credential::hold_confirmation(&state.urls, &query.token, jar);
            (jar, Redirect::to(&location)).into_response()
        }
    })
}

pub async fn dashboard_delete(
    State(state): State<AppState>,
    OwnedPublication { publication, .. }: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
) -> AppResult<StatusCode> {
    comments::in_publication(&state.db, &publication.id, &id).await?.ok_or(AppError::NotFound)?;
    lifecycle::remove(&state.db, &id).await?;
    Ok(StatusCode::OK)
}
