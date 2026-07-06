pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_users_table;
mod m20250101_000002_create_newsletters_table;
mod m20250101_000003_create_refresh_tokens_table;
mod m20250101_000004_schema_updates;
mod m20260627_000005_add_rendered_to_newsletters;
mod m20260627_000006_change_sent_to_sent_at;
mod m20260627_000007_create_subscribers_table;

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
        ]
    }
}
