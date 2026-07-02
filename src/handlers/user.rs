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
    views,
};

pub async fn profile(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Extension(ctx): Extension<UserContext>,
) -> Markup {
    views::user::profile(&handle, ctx.is_authenticated(), &state.urls)
}

pub async fn newsletter(
    State(state): State<AppState>,
    UsernameSubdomain(handle): UsernameSubdomain,
    Path(slug): Path<String>,
) -> Result<Markup, StatusCode> {
    let user = User::find()
        .filter(user::Column::Handle.eq(&handle))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let newsletter = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&user.id))
        .filter(newsletter::Column::Slug.eq(&slug))
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(views::user::newsletter(newsletter))
}
