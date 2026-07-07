use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use nanoid::nanoid;
use maud::Markup;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use crate::{
    auth::extractors::AuthenticatedUser,
    models::newsletter::{self, Entity as Newsletter},
    state::AppState,
    views::{self, PageContext},
};

pub async fn index(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Markup {
    views::dashboard::index(&PageContext::from_user(&user, state.urls.clone()))
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

pub async fn post_newsletters(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Response {
    let id = nanoid!(14);
    let now = chrono::Utc::now().fixed_offset();

    let new_newsletter = newsletter::ActiveModel {
        id: Set(id.clone()),
        user_id: Set(user.id),
        title: Set("Untitled".to_string()),
        slug: Set(id.clone()),
        subtitle: Set(None),
        content: Set(serde_json::json!({ "type": "doc", "content": [] })),
        sent_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        rendered: Set(None)
    };

    match new_newsletter.insert(&state.db).await {
        Ok(newsletter) => Redirect::to(&format!("/edit/{}", newsletter.id)).into_response(),
        Err(e) => {
            eprintln!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}
