use axum::{extract::{Path, State}, http::StatusCode, Extension};
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::{
    auth::{context::UserContext, extractors::AuthenticatedUser}, models::{
        newsletter::{self, Entity as Newsletter},
        user::{self, Entity as User},
    }, services::subdomain::UsernameSubdomain, state::AppState, views::{self, pages::error404, PageContext},
};

pub async fn profile(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Extension(ctx): Extension<UserContext>,
) -> Markup {
    let page_ctx = PageContext::public(&ctx, state.urls.clone())
        .with_page_owner(&handle);
    views::user::profile(&page_ctx)
}

pub async fn newsletter(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Path(slug): Path<String>,
    Extension(ctx): Extension<UserContext>,
) -> (StatusCode, Markup) {
    let page_ctx = PageContext::public(&ctx, state.urls.clone())
        .with_page_owner(&handle);

    let owner = match User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
    {
        Ok(Some(u)) => u,
        _ => return (StatusCode::NOT_FOUND, error404::user_404(&page_ctx)),
    };

    let newsletter = match Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&owner.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .filter(newsletter::Column::SentAt.is_not_null())
        .one(&state.db)
        .await
    {
        Ok(Some(n)) => n,
        _ => return (StatusCode::NOT_FOUND, error404::user_404(&page_ctx)),
    };

    (StatusCode::OK, views::user::newsletter(newsletter, &page_ctx))
}

pub async fn get_newsletters(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let mut newsletters = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&user.id))
        .filter(newsletter::Column::SentAt.is_not_null())
        .all(&state.db)
        .await
        .unwrap_or_default();

    newsletters.sort_unstable_by_key(|n| std::cmp::Reverse(n.sent_at));

    let user_base = state.urls.user(&user.handle);
    views::user::newsletters(newsletters, &user_base)
}
