use axum::extract::State;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    models::{
        newsletter::{self, Entity as Newsletter},
        user::Entity as User,
    },
    state::AppState,
    views::sitemap::{self, Xml},
};

pub async fn index(State(state): State<AppState>) -> Xml {
    sitemap::index(&state.urls.base())
}

pub async fn pages(State(state): State<AppState>) -> Xml {
    sitemap::pages(&state.urls.base())
}

pub async fn users(State(state): State<AppState>) -> Xml {
    let users = User::find()
        .all(&state.db)
        .await
        .unwrap_or_default();

    sitemap::users(&users, &state.urls)
}

pub async fn newsletters(State(state): State<AppState>) -> Xml {
    let items: Vec<_> = Newsletter::find()
        .filter(newsletter::Column::SentAt.is_not_null())
        .find_also_related(User)
        .all(&state.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(n, u)| u.map(|u| (n, u)))
        .collect();

    sitemap::newsletters(&items, &state.urls)
}
