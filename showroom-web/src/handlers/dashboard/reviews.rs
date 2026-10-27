use std::{collections::HashSet, ops::Range};

use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State},
    http::{HeaderName, HeaderValue, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use boutique::{AppError, AppResult};
use chrono::{Duration, Utc};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
    sea_query::Expr,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    document::{Block, Document, merge},
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
        review::{self, Entity as Review, ReviewSummary},
        review_link::{self, Entity as ReviewLink},
    },
    services::publication::{OwnedPublication, find_newsletter},
    state::AppState,
    views::{
        self,
        dashboard::reviews::{NewReviewForm, PersonRow, ResultsView, ReviewGroup},
    },
};

const REVIEW_DAYS: i64 = 30;
const MAX_NAME: usize = 60;
const MAX_NOTE: usize = 1_000;
const MAX_REPLY: usize = 2_000;
const MAX_PEOPLE: usize = 20;

fn form_pairs(body: &Bytes) -> Vec<(String, String)> {
    url::form_urlencoded::parse(body).into_owned().collect()
}

fn form_value(pairs: &[(String, String)], key: &str) -> Option<String> {
    pairs.iter().find(|(name, _)| name == key).map(|(_, value)| value.trim().to_string()).filter(|value| !value.is_empty())
}

fn form_values(pairs: &[(String, String)], key: &str) -> Vec<String> {
    pairs.iter().filter(|(name, _)| name == key).map(|(_, value)| value.trim().to_string()).filter(|value| !value.is_empty()).collect()
}

fn too_long(text: &Option<String>, max: usize) -> bool {
    text.as_ref().is_some_and(|text| text.chars().count() > max)
}

pub fn link_url(state: &AppState, slug: &str, token: &str) -> String {
    format!("{}/{}/review/{}", state.urls.app(), slug, token)
}

async fn reviews_with_links(state: &AppState, newsletter_id: &str) -> AppResult<Vec<(review::Model, Vec<review_link::Model>)>> {
    Ok(Review::find()
        .filter(review::Column::NewsletterId.eq(newsletter_id))
        .filter(review::Column::ExpiresAt.gt(Utc::now().fixed_offset()))
        .order_by_desc(review::Column::CreatedAt)
        .find_with_related(ReviewLink)
        .all(&state.db)
        .await?)
}

pub async fn review_summaries(state: &AppState, newsletter_id: &str) -> AppResult<Vec<ReviewSummary>> {
    Ok(reviews_with_links(state, newsletter_id)
        .await?
        .into_iter()
        .map(|(review, links)| ReviewSummary {
            section_id: review.section_id,
            people: links.len(),
            returned: links.iter().filter(|link| link.has_returned()).count(),
        })
        .collect())
}

fn reviews_url(slug: &str, newsletter_id: &str) -> String {
    format!("{}/reviews", editor_url(slug, newsletter_id))
}

async fn find_owned_review(state: &AppState, owned: &OwnedPublication, review_id: &str) -> AppResult<(review::Model, newsletter::Model)> {
    let (review, newsletter) = Review::find_by_id(review_id)
        .find_also_related(Newsletter)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    let newsletter = newsletter.filter(|newsletter| newsletter.publication_id == owned.publication.id).ok_or(AppError::NotFound)?;
    Ok((review, newsletter))
}

async fn find_owned_link(state: &AppState, owned: &OwnedPublication, review_id: &str, link_id: &str) -> AppResult<(review::Model, newsletter::Model, review_link::Model)> {
    let (review, newsletter) = find_owned_review(state, owned, review_id).await?;
    let link = ReviewLink::find_by_id(link_id)
        .filter(review_link::Column::ReviewId.eq(&review.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok((review, newsletter, link))
}

fn editor_url(slug: &str, newsletter_id: &str) -> String {
    format!("/{slug}/edit/{newsletter_id}")
}

fn link_path(slug: &str, review_id: &str, link_id: &str) -> String {
    format!("/{slug}/reviews/{review_id}/links/{link_id}")
}

#[derive(Deserialize)]
pub struct NewReviewQuery {
    section: Option<String>,
}

struct Snapshot {
    blocks: Vec<Block>,
    after_section_id: Option<String>,
}

fn snapshot(newsletter: &newsletter::Model, section_id: Option<&str>) -> Option<Snapshot> {
    let document = Document::from_stored(&newsletter.content);
    let span = document.scope(section_id)?;
    let after_section_id = section_id.and_then(|_| document.heading_before(span.start));
    Some(Snapshot { blocks: document.content[span].to_vec(), after_section_id })
}

fn render_new_review(owned: OwnedPublication, state: &AppState, newsletter: &newsletter::Model, form: NewReviewForm) -> AppResult<Response> {
    let section_id = form.section_id.clone();
    let Some(Snapshot { blocks, .. }) = snapshot(newsletter, section_id.as_deref()) else {
        return Err(AppError::NotFound);
    };
    let status = if form.error.is_some() { StatusCode::UNPROCESSABLE_ENTITY } else { StatusCode::OK };
    let ctx = owned.into_context(state.urls.clone());
    Ok((status, [(header::CACHE_CONTROL, "no-store")], views::dashboard::reviews::new_review(&ctx, newsletter, &Document { content: blocks }, &form)).into_response())
}

pub async fn new_review_page(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((_, id)): Path<(String, String)>,
    Query(query): Query<NewReviewQuery>,
) -> AppResult<Response> {
    let newsletter = find_newsletter(&state.db, &owned.publication.id, &id).await?;
    let form = NewReviewForm { section_id: query.section.filter(|section| !section.is_empty()), ..NewReviewForm::default() };
    render_new_review(owned, &state, &newsletter, form)
}

pub async fn create_review(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, id)): Path<(String, String)>,
    body: Bytes,
) -> AppResult<Response> {
    let newsletter = find_newsletter(&state.db, &owned.publication.id, &id).await?;
    let pairs = form_pairs(&body);
    let mut form = NewReviewForm {
        section_id: form_value(&pairs, "section"),
        note: form_value(&pairs, "note"),
        names: form_values(&pairs, "reviewer"),
        error: None,
    };

    let mut seen = HashSet::new();
    form.names.retain(|name| seen.insert(name.to_lowercase()));
    form.error = if form.names.is_empty() {
        Some("Add at least one person.".to_string())
    } else if form.names.len() > MAX_PEOPLE {
        Some(format!("You can add up to {MAX_PEOPLE} people at a time."))
    } else if form.names.iter().any(|name| name.chars().count() > MAX_NAME) {
        Some(format!("Names can be at most {MAX_NAME} characters."))
    } else if too_long(&form.note, MAX_NOTE) {
        Some(format!("The note can be at most {MAX_NOTE} characters."))
    } else {
        None
    };
    if form.error.is_some() {
        return render_new_review(owned, &state, &newsletter, form);
    }

    let Some(Snapshot { blocks, after_section_id }) = snapshot(&newsletter, form.section_id.as_deref()) else {
        form.error = Some("This section no longer exists. Go back to the editor and try again.".to_string());
        return render_new_review(owned, &state, &newsletter, form);
    };

    let now = Utc::now().fixed_offset();
    let review_id = nanoid!(14);
    let transaction = state.db.begin().await?;
    review::ActiveModel {
        id: Set(review_id.clone()),
        newsletter_id: Set(newsletter.id.clone()),
        section_id: Set(form.section_id.clone()),
        after_section_id: Set(after_section_id),
        note: Set(form.note.clone()),
        base: Set(Document { content: blocks }.to_value()),
        created_at: Set(now),
        expires_at: Set(now + Duration::days(REVIEW_DAYS)),
    }
    .insert(&transaction)
    .await?;
    for name in &form.names {
        new_link(&review_id, name, now).insert(&transaction).await?;
    }
    transaction.commit().await?;

    Ok(Redirect::to(&reviews_url(&slug, &newsletter.id)).into_response())
}

fn new_link(review_id: &str, name: &str, now: chrono::DateTime<chrono::FixedOffset>) -> review_link::ActiveModel {
    review_link::ActiveModel {
        id: Set(nanoid!(14)),
        review_id: Set(review_id.to_string()),
        token: Set(nanoid!(32)),
        reviewer_name: Set(name.to_string()),
        proposed: Set(None),
        reply: Set(None),
        revision: Set(0),
        opened_at: Set(None),
        sent_at: Set(None),
        applied_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    }
}

fn review_group(state: &AppState, slug: &str, review: review::Model, links: Vec<review_link::Model>) -> ReviewGroup {
    ReviewGroup {
        label: review.label(),
        created_at: review.created_at,
        people: links
            .into_iter()
            .map(|link| PersonRow {
                url: link_url(state, slug, &link.token),
                results_url: link_path(slug, &review.id, &link.id),
                delete_url: format!("{}/delete", link_path(slug, &review.id, &link.id)),
                link,
            })
            .collect(),
    }
}

pub async fn newsletter_reviews(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, id)): Path<(String, String)>,
) -> AppResult<Response> {
    let newsletter = find_newsletter(&state.db, &owned.publication.id, &id).await?;
    let groups: Vec<ReviewGroup> = reviews_with_links(&state, &newsletter.id)
        .await?
        .into_iter()
        .map(|(review, mut links)| {
            links.sort_by(|a, b| a.created_at.cmp(&b.created_at).then_with(|| a.reviewer_name.cmp(&b.reviewer_name)));
            review_group(&state, &slug, review, links)
        })
        .collect();
    if groups.is_empty() {
        return Ok(Redirect::to(&format!("{}/review", editor_url(&slug, &id))).into_response());
    }
    let ctx = owned.into_context(state.urls.clone());
    Ok(([(header::CACHE_CONTROL, "no-store")], views::dashboard::reviews::index(&ctx, &groups, &editor_url(&slug, &id))).into_response())
}

pub async fn delete_link(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, review_id, link_id)): Path<(String, String, String)>,
) -> AppResult<Response> {
    let (review, newsletter, link) = find_owned_link(&state, &owned, &review_id, &link_id).await?;
    ReviewLink::delete_by_id(link.id).exec(&state.db).await?;
    if review.find_related(ReviewLink).one(&state.db).await?.is_none() {
        Review::delete_by_id(review.id).exec(&state.db).await?;
    }
    Ok(Redirect::to(&reviews_url(&slug, &newsletter.id)).into_response())
}

struct Comparison {
    proposed: Vec<Block>,
    span: Option<Range<usize>>,
    current: Vec<Block>,
    planned: Vec<merge::Planned>,
}

fn compare(review: &review::Model, newsletter: &newsletter::Model, link: &review_link::Model) -> Comparison {
    let base = review.blocks();
    let proposed = link.proposed.as_ref().map(|value| Document::from_stored(value).trimmed().content).unwrap_or_else(|| base.clone());
    let document = Document::from_stored(&newsletter.content);
    let span = document.scope(review.section_id.as_deref());
    let current = span.clone().map_or_else(|| base.clone(), |span| document.content[span].to_vec());
    let planned = merge::plan(&base, &proposed, &current);
    Comparison { proposed, span, current, planned }
}

#[derive(Deserialize)]
pub struct ResultsQuery {
    applied: Option<usize>,
    stale: Option<bool>,
}

pub async fn link_results(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, review_id, link_id)): Path<(String, String, String)>,
    Query(query): Query<ResultsQuery>,
) -> AppResult<Response> {
    let (review, newsletter, link) = find_owned_link(&state, &owned, &review_id, &link_id).await?;
    let comparison = compare(&review, &newsletter, &link);
    let path = link_path(&slug, &review.id, &link.id);
    let view = ResultsView {
        reviews_url: reviews_url(&slug, &newsletter.id),
        editor_url: editor_url(&slug, &newsletter.id),
        apply_url: format!("{path}/apply"),
        restore_url: format!("{path}/restore"),
        proposed: &comparison.proposed,
        current: &comparison.current,
        planned: &comparison.planned,
        deleted: comparison.span.is_none(),
        newsletter_revision: newsletter.revision,
        link_revision: link.revision,
        applied: query.applied,
        stale: query.stale.unwrap_or(false),
        link: &link,
    };
    let ctx = owned.into_context(state.urls.clone());
    Ok(([(header::CACHE_CONTROL, "no-store")], views::dashboard::reviews::results(&ctx, &view)).into_response())
}

fn revisions_match(pairs: &[(String, String)], newsletter: &newsletter::Model, link: &review_link::Model) -> bool {
    let parse = |key| form_value(pairs, key).and_then(|value| value.parse::<i32>().ok());
    parse("revision") == Some(newsletter.revision) && parse("link_revision") == Some(link.revision)
}

async fn save_applied(state: &AppState, newsletter: &newsletter::Model, link: &review_link::Model, document: &Document) -> AppResult<bool> {
    let now = Utc::now().fixed_offset();
    let transaction = state.db.begin().await?;
    let result = Newsletter::update_many()
        .col_expr(newsletter::Column::Content, Expr::value(document.to_value()))
        .col_expr(newsletter::Column::UpdatedAt, Expr::value(now))
        .col_expr(newsletter::Column::Revision, Expr::col(newsletter::Column::Revision).add(1))
        .filter(newsletter::Column::Id.eq(&newsletter.id))
        .filter(newsletter::Column::Revision.eq(newsletter.revision))
        .exec(&transaction)
        .await?;
    if result.rows_affected == 0 {
        return Ok(false);
    }
    ReviewLink::update_many()
        .col_expr(review_link::Column::AppliedAt, Expr::value(now))
        .filter(review_link::Column::Id.eq(&link.id))
        .exec(&transaction)
        .await?;
    transaction.commit().await?;
    Ok(true)
}

pub async fn apply_link(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, review_id, link_id)): Path<(String, String, String)>,
    body: Bytes,
) -> AppResult<Response> {
    let (review, newsletter, link) = find_owned_link(&state, &owned, &review_id, &link_id).await?;
    let results = link_path(&slug, &review.id, &link.id);
    let pairs = form_pairs(&body);
    if !revisions_match(&pairs, &newsletter, &link) {
        return Ok(Redirect::to(&format!("{results}?stale=true")).into_response());
    }

    let comparison = compare(&review, &newsletter, &link);
    let Some(span) = comparison.span.clone() else {
        return Ok(Redirect::to(&results).into_response());
    };
    let selected: HashSet<usize> = form_values(&pairs, "change")
        .iter()
        .filter_map(|value| value.parse().ok())
        .filter(|index| comparison.planned.get(*index).is_some_and(merge::Planned::can_apply))
        .collect();
    if selected.is_empty() {
        return Ok(Redirect::to(&format!("{results}?applied=0")).into_response());
    }

    let merged = merge::apply(&comparison.current, &comparison.proposed, &comparison.planned, |index| selected.contains(&index));
    let mut document = Document::from_stored(&newsletter.content);
    document.content.splice(span, merged);
    if !save_applied(&state, &newsletter, &link, &document).await? {
        return Ok(Redirect::to(&format!("{results}?stale=true")).into_response());
    }
    Ok(Redirect::to(&format!("{results}?applied={}", selected.len())).into_response())
}

pub async fn restore_link(
    State(state): State<AppState>,
    owned: OwnedPublication,
    Path((slug, review_id, link_id)): Path<(String, String, String)>,
    body: Bytes,
) -> AppResult<Response> {
    let (review, newsletter, link) = find_owned_link(&state, &owned, &review_id, &link_id).await?;
    let results = link_path(&slug, &review.id, &link.id);
    if !revisions_match(&form_pairs(&body), &newsletter, &link) {
        return Ok(Redirect::to(&format!("{results}?stale=true")).into_response());
    }

    let comparison = compare(&review, &newsletter, &link);
    if comparison.span.is_some() || comparison.proposed.is_empty() {
        return Ok(Redirect::to(&results).into_response());
    }
    let mut document = Document::from_stored(&newsletter.content);
    let index = document.restore_index(review.after_section_id.as_deref());
    document.content.splice(index..index, comparison.proposed);
    if !save_applied(&state, &newsletter, &link, &document).await? {
        return Ok(Redirect::to(&format!("{results}?stale=true")).into_response());
    }
    Ok(Redirect::to(&format!("{results}?applied=1")).into_response())
}

struct PublicLink {
    link: review_link::Model,
    review: review::Model,
    newsletter: newsletter::Model,
    publication: publication::Model,
}

async fn find_public_link(state: &AppState, slug: &str, token: &str) -> AppResult<PublicLink> {
    let publication = Publication::find().filter(publication::Column::Slug.eq(slug)).one(&state.db).await?.ok_or(AppError::NotFound)?;
    let (link, review) = ReviewLink::find()
        .filter(review_link::Column::Token.eq(token))
        .find_also_related(Review)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    let review = review.filter(|review| review.expires_at > Utc::now().fixed_offset()).ok_or(AppError::NotFound)?;
    let newsletter = Newsletter::find_by_id(&review.newsletter_id)
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(PublicLink { link, review, newsletter, publication })
}

fn private_headers() -> [(HeaderName, HeaderValue); 3] {
    [
        (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
        (header::REFERRER_POLICY, HeaderValue::from_static("no-referrer")),
        (HeaderName::from_static("x-robots-tag"), HeaderValue::from_static("noindex, nofollow")),
    ]
}

pub async fn get_review_page(
    State(state): State<AppState>,
    Path((slug, token)): Path<(String, String)>,
) -> AppResult<Response> {
    let found = match find_public_link(&state, &slug, &token).await {
        Err(AppError::NotFound) => return Ok((StatusCode::NOT_FOUND, private_headers(), views::review::inactive()).into_response()),
        found => found?,
    };
    if found.link.opened_at.is_none() {
        ReviewLink::update_many()
            .col_expr(review_link::Column::OpenedAt, Expr::value(Utc::now().fixed_offset()))
            .filter(review_link::Column::Id.eq(&found.link.id))
            .exec(&state.db)
            .await?;
    }
    let page = views::review::review(&found.publication, &found.newsletter, &found.review, &found.link, &format!("/{slug}/review/{token}"));
    Ok((private_headers(), page).into_response())
}

#[boutique::ts]
#[derive(Deserialize)]
pub struct ReviewSaveRequest {
    #[ts(type = "unknown")]
    content: Value,
    revision: i32,
    #[ts(optional)]
    reply: Option<String>,
}

#[boutique::ts]
#[derive(Serialize)]
pub struct ReviewSaveResponse {
    revision: i32,
}

fn validate_proposal(review: &review::Model, content: &Value) -> AppResult<Document> {
    let document = Document::from_value(content)
        .map_err(|error| AppError::message(format!("The content is not valid: {error}")))?
        .trimmed();

    if let Some(section_id) = &review.section_id {
        let keeps_heading = matches!(
            document.content.first(),
            Some(Block::Heading { attrs, .. }) if attrs.id.as_deref() == Some(section_id.as_str())
        );
        if !keeps_heading {
            return Err(AppError::message("The section must start with its heading."));
        }
    }

    let allowed: HashSet<String> = review.blocks().iter().flat_map(Block::media_keys).collect();
    if document.content.iter().flat_map(Block::media_keys).any(|key| !allowed.contains(&key)) {
        return Err(AppError::message("Reviewers can't add new images or embeds."));
    }

    Ok(document)
}

async fn save_proposal(state: &AppState, found: &PublicLink, body: ReviewSaveRequest, send: bool) -> AppResult<Response> {
    let document = validate_proposal(&found.review, &body.content)?;
    let reply = body.reply.map(|reply| reply.trim().to_string()).filter(|reply| !reply.is_empty());
    if too_long(&reply, MAX_REPLY) {
        return Err(AppError::message(format!("The message can be at most {MAX_REPLY} characters.")));
    }
    let now = Utc::now().fixed_offset();

    let mut update = ReviewLink::update_many()
        .col_expr(review_link::Column::Proposed, Expr::value(document.to_value()))
        .col_expr(review_link::Column::Revision, Expr::col(review_link::Column::Revision).add(1))
        .filter(review_link::Column::Id.eq(&found.link.id))
        .filter(review_link::Column::Revision.eq(body.revision));
    update = if send {
        update
            .col_expr(review_link::Column::Reply, Expr::value(reply))
            .col_expr(review_link::Column::SentAt, Expr::value(now))
            .col_expr(review_link::Column::UpdatedAt, Expr::value(now))
    } else {
        update.col_expr(review_link::Column::UpdatedAt, Expr::value(now))
    };

    if update.exec(&state.db).await?.rows_affected == 0 {
        let current = ReviewLink::find_by_id(&found.link.id).one(&state.db).await?.ok_or(AppError::NotFound)?;
        return Ok((StatusCode::CONFLICT, Json(ReviewSaveResponse { revision: current.revision })).into_response());
    }

    Ok((private_headers(), Json(ReviewSaveResponse { revision: body.revision + 1 })).into_response())
}

pub async fn put_review_json(
    State(state): State<AppState>,
    Path((slug, token)): Path<(String, String)>,
    Json(body): Json<ReviewSaveRequest>,
) -> AppResult<Response> {
    let found = match find_public_link(&state, &slug, &token).await {
        Err(AppError::NotFound) => return Ok((StatusCode::FORBIDDEN, private_headers()).into_response()),
        found => found?,
    };
    save_proposal(&state, &found, body, false).await
}

pub async fn post_review_send(
    State(state): State<AppState>,
    Path((slug, token)): Path<(String, String)>,
    Json(body): Json<ReviewSaveRequest>,
) -> AppResult<Response> {
    let found = match find_public_link(&state, &slug, &token).await {
        Err(AppError::NotFound) => return Ok((StatusCode::FORBIDDEN, private_headers()).into_response()),
        found => found?,
    };
    save_proposal(&state, &found, body, true).await
}
