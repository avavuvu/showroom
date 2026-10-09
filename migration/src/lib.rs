pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_users_table;
mod m20250101_000002_create_newsletters_table;
mod m20250101_000003_create_refresh_tokens_table;
mod m20250101_000004_schema_updates;
mod m20260627_000005_add_rendered_to_newsletters;
mod m20260627_000006_change_sent_to_sent_at;
mod m20260627_000007_create_subscribers_table;
mod m20260909_125444_create_publications;
mod m20260929_000008_create_sessions;
mod m20261008_000009_newsletter_revisions;
mod m20261008_000010_publication_greeting;
mod m20261009_000011_newsletter_deliveries;
mod m20261009_000012_publication_images;
mod m20261009_000013_publication_picture_history;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_users_table::Migration),
            Box::new(m20250101_000002_create_newsletters_table::Migration),
            Box::new(m20250101_000003_create_refresh_tokens_table::Migration),
            Box::new(m20250101_000004_schema_updates::Migration),
            Box::new(m20260627_000005_add_rendered_to_newsletters::Migration),
            Box::new(m20260627_000006_change_sent_to_sent_at::Migration),
            Box::new(m20260627_000007_create_subscribers_table::Migration),
            Box::new(m20260909_125444_create_publications::Migration),
            Box::new(m20260929_000008_create_sessions::Migration),
            Box::new(m20261008_000009_newsletter_revisions::Migration),
            Box::new(m20261008_000010_publication_greeting::Migration),
            Box::new(m20261009_000011_newsletter_deliveries::Migration),
            Box::new(m20261009_000012_publication_images::Migration),
            Box::new(m20261009_000013_publication_picture_history::Migration),
        ]
    }
}
