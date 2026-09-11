mod create_users;
mod create_refresh_tokens;

use sea_orm_migration::MigrationTrait;

/// migrations for the tables boutique owns. append these to the app's migrator
/// before any app migration that references `users`.
pub fn all() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(create_users::Migration),
        Box::new(create_refresh_tokens::Migration),
    ]
}
