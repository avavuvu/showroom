use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}};
use maud::Markup;
use sea_orm::{ColumnTrait, QueryFilter, QueryOrder};
use axum_extra::extract::cookie::CookieJar;
use boutique::{AppResult, AuthenticatedUser};
use serde::Deserialize;
use crate::{
    handlers::{comments::{Thread, exchange_footer}, feeds::{Format, html_headers, markdown_response}},
    models::newsletter::{self, Entity as Newsletter},
    services::subdomain::CurrentPublication, state::AppState, views::{self, pages::error404, PageContext, Viewer},
};

#[derive(Deserialize)]
pub struct CommenterQuery {
    pub commenter: Option<String>,
}

pub async fn profile(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    viewer: Viewer,
) -> AppResult<Markup> {
    let newsletters = Newsletter::find_summaries()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .order_by_desc(newsletter::Column::PublishedAt)
        .all(&state.db)
        .await?;

    let page_ctx = PageContext::public(viewer, state.urls.clone()).with_publication(publication);
    Ok(views::user::profile::index(&page_ctx, &newsletters))
}

pub async fn newsletter(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Path(slug): Path<String>,
    Query(query): Query<CommenterQuery>,
    headers: HeaderMap,
    jar: CookieJar,
    viewer: Viewer,
) -> AppResult<Response> {
    let (slug, format) = Format::negotiate(&slug, &headers);
    let newsletter = Newsletter::find_published(&publication.id, slug).one(&state.db).await?;

    if let (Some(newsletter), Some(token)) = (&newsletter, query.commenter.as_deref()) {
        return exchange_footer(&state, &publication, newsletter, token, jar).await;
    }

    let thread = match &newsletter {
        Some(newsletter) if format == Format::Html => Thread::load(&state, &jar, viewer.as_ref().map(|AuthenticatedUser(user)| user), publication.clone(), newsletter.clone()).await?,
        _ => None,
    };
    let discussion = match &thread {
        Some(thread) => Some(thread.discussion().await?),
        None => None,
    };
    let page_ctx = PageContext::public(viewer, state.urls.clone()).with_publication(publication);

    let Some(newsletter) = newsletter else {
        return Ok((StatusCode::NOT_FOUND, error404::publication_404(&page_ctx)).into_response());
    };
    let url = views::user::newsletter_url(&page_ctx.publication_url(), &newsletter);
    if format == Format::Markdown {
        return Ok(markdown_response(page_ctx.publication(), &newsletter, &url));
    }
    let mut response = views::user::newsletter(newsletter, &page_ctx, discussion.as_ref()).into_response();
    html_headers(&mut response, &url);
    Ok(response)
}
