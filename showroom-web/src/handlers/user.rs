use axum::{extract::{Path, State}, http::StatusCode, Extension};
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use boutique::UserContext;
use crate::{
    models::newsletter::{self, Entity as Newsletter},
    services::subdomain::CurrentPublication, state::AppState, views::{self, pages::error404, PageContext},
};

pub async fn profile(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Extension(ctx): Extension<UserContext>,
) -> Markup {
    let mut newsletters = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::SentAt.is_not_null())
        .all(&state.db)
        .await
        .unwrap_or_default();

    newsletters.sort_unstable_by_key(|n| std::cmp::Reverse(n.sent_at));

    let page_ctx = PageContext::public(&ctx, state.urls.clone()).with_publication(publication);
    views::user::profile(&page_ctx, &newsletters)
}

pub async fn newsletter(
    State(state): State<AppState>,
    CurrentPublication(publication): CurrentPublication,
    Path(slug): Path<String>,
    Extension(ctx): Extension<UserContext>,
) -> (StatusCode, Markup) {
    let newsletter = Newsletter::find()
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .filter(newsletter::Column::SentAt.is_not_null())
        .one(&state.db)
        .await;

    let page_ctx = PageContext::public(&ctx, state.urls.clone()).with_publication(publication);

    match newsletter {
        Ok(Some(n)) => (StatusCode::OK, views::user::newsletter(n, &page_ctx)),
        _ => (StatusCode::NOT_FOUND, error404::publication_404(&page_ctx)),
    }
}
