use axum::{extract::{Path, State}, http::StatusCode};
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use boutique::AppResult;
use crate::{
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
    Ok(views::user::profile(&page_ctx, &newsletters))
}

pub async fn newsletter(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Path(slug): Path<String>,
    viewer: Viewer,
) -> AppResult<(StatusCode, Markup)> {
    let newsletter = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .one(&state.db)
        .await?;

    let page_ctx = PageContext::public(viewer, state.urls.clone()).with_publication(publication);

    Ok(match newsletter {
        Some(n) => (StatusCode::OK, views::user::newsletter(n, &page_ctx)),
        None => (StatusCode::NOT_FOUND, error404::publication_404(&page_ctx)),
    })
}
