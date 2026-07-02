use axum::{extract::{Path, State}, http::StatusCode, Extension};
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::{
    auth::context::UserContext,
    models::{
        newsletter::{self, Entity as Newsletter},
        user::{self, Entity as User},
    },
    services::subdomain::UsernameSubdomain,
    state::AppState,
    views::{self, PageContext},
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
) -> Result<Markup, StatusCode> {
    let owner = User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let newsletter = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&owner.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let page_ctx = PageContext::public(&ctx, state.urls.clone())
        .with_page_owner(&handle);

    Ok(views::user::newsletter(newsletter, &page_ctx))
}
