use axum::{extract::{Path, State}, http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}};
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use boutique::AppResult;
use crate::{
    handlers::feeds::{Format, html_headers, markdown_response},
    models::newsletter::{self, Entity as Newsletter},
    services::subdomain::CurrentPublication, state::AppState, views::{self, pages::error404, PageContext, Viewer},
};

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
    headers: HeaderMap,
    viewer: Viewer,
) -> AppResult<Response> {
    let (slug, format) = Format::negotiate(&slug, &headers);
    let newsletter = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::Slug.eq(slug))
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .one(&state.db)
        .await?;

    let page_ctx = PageContext::public(viewer, state.urls.clone()).with_publication(publication);

    let Some(newsletter) = newsletter else {
        return Ok((StatusCode::NOT_FOUND, error404::publication_404(&page_ctx)).into_response());
    };
    let url = views::user::newsletter_url(&page_ctx.publication_url(), &newsletter);
    if format == Format::Markdown {
        return Ok(markdown_response(page_ctx.publication(), &newsletter, &url));
    }
    let mut response = views::user::newsletter(newsletter, &page_ctx).into_response();
    html_headers(&mut response, &url);
    Ok(response)
}
