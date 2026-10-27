use axum::extract::State;
use boutique::AppResult;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::{
    models::newsletter::{self, Entity as Newsletter},
    services::subdomain::CurrentPublication,
    state::AppState,
    views::sitemap::{self, Entry, Xml},
};

pub async fn site(State(state): State<AppState>) -> Xml {
    let base = state.urls.base();
    sitemap::urlset(&[Entry { loc: format!("{base}/"), lastmod: None }, Entry { loc: format!("{base}/about"), lastmod: None }])
}

pub async fn publication(State(state): State<AppState>, CurrentPublication(publication): CurrentPublication) -> AppResult<Xml> {
    let newsletters: Vec<(String, chrono::DateTime<chrono::FixedOffset>)> = Newsletter::find()
        .select_only()
        .column(newsletter::Column::Slug)
        .column(newsletter::Column::UpdatedAt)
        .filter(newsletter::Column::PublicationId.eq(&publication.id))
        .filter(newsletter::Column::PublishedAt.is_not_null())
        .order_by_desc(newsletter::Column::PublishedAt)
        .into_tuple()
        .all(&state.db)
        .await?;

    let url = state.urls.publication(&publication.slug);
    let mut entries = Vec::new();
    if let Some(latest) = newsletters.iter().map(|(_, updated_at)| *updated_at).max() {
        entries.push(Entry { loc: format!("{url}/"), lastmod: Some(latest) });
    }
    entries.extend(newsletters.into_iter().map(|(slug, updated_at)| Entry { loc: format!("{url}/{slug}"), lastmod: Some(updated_at) }));
    Ok(sitemap::urlset(&entries))
}
