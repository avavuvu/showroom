use axum::extract::State;
use boutique::AppResult;
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};

use crate::{
    models::{
        newsletter::{self, Entity as Newsletter},
        publication::{self, Entity as Publication},
    },
    state::AppState,
    views::sitemap::{self, NewsletterEntry, Xml},
};

pub async fn index(State(state): State<AppState>) -> Xml {
    sitemap::index(&state.urls.base())
}

pub async fn pages(State(state): State<AppState>) -> Xml {
    sitemap::pages(&state.urls.base())
}

pub async fn publications(State(state): State<AppState>) -> AppResult<Xml> {
    let slugs: Vec<String> = Publication::find()
        .select_only()
        .column(publication::Column::Slug)
        .into_tuple()
        .all(&state.db)
        .await?;

    Ok(sitemap::publications(&slugs, &state.urls))
}

pub async fn newsletters(State(state): State<AppState>) -> AppResult<Xml> {
    let entries = Newsletter::find()
        .select_only()
        .column(publication::Column::Slug)
        .column(newsletter::Column::Slug)
        .column(newsletter::Column::UpdatedAt)
        .join(JoinType::InnerJoin, newsletter::Relation::Publication.def())
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .into_tuple()
        .all(&state.db)
        .await?
        .into_iter()
        .map(|(publication_slug, slug, updated_at)| NewsletterEntry { publication_slug, slug, updated_at })
        .collect::<Vec<_>>();

    Ok(sitemap::newsletters(&entries, &state.urls))
}
