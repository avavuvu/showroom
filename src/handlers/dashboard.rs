use axum::extract::State;
use maud::Markup;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::{
    auth::extractors::AuthenticatedUser,
    models::newsletter::{self, Entity as Newsletter},
    state::AppState,
    views,
};

pub async fn index(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Markup {
    views::dashboard::index(&user, &state.urls.base())
}

pub async fn get_newsletters(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let mut newsletters = Newsletter::find()
        .filter(newsletter::Column::UserId.eq(&user.id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    newsletters.sort_unstable_by_key(|n| std::cmp::Reverse(n.updated_at));

    let user_base = state.urls.user(&user.handle);
    views::dashboard::newsletters(newsletters, &user_base)
}
