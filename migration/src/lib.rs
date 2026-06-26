pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_users_table;
mod m20250101_000002_create_newsletters_table;
mod m20250101_000003_create_refresh_tokens_table;
mod m20250101_000004_schema_updates;
mod m20250101_000005_add_rendered_html_to_newsletters;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_users_table::Migration),
            Box::new(m20250101_000002_create_newsletters_table::Migration),
            Box::new(m20250101_000003_create_refresh_tokens_table::Migration),
            Box::new(m20250101_000004_schema_updates::Migration),
            Box::new(m20250101_000005_add_rendered_html_to_newsletters::Migration),
        ]
    }
}
