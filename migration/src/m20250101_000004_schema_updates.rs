use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Backfill any NULL handles before making the column NOT NULL
        conn.execute_unprepared("UPDATE users SET handle = id WHERE handle IS NULL").await?;
        conn.execute_unprepared("ALTER TABLE users ALTER COLUMN handle SET NOT NULL").await?;

        // Backfill any NULL updated_at before making the column NOT NULL
        conn.execute_unprepared("UPDATE newsletters SET updated_at = created_at WHERE updated_at IS NULL").await?;
        conn.execute_unprepared("ALTER TABLE newsletters ALTER COLUMN updated_at SET NOT NULL").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        conn.execute_unprepared("ALTER TABLE users ALTER COLUMN handle DROP NOT NULL").await?;
        conn.execute_unprepared("ALTER TABLE newsletters ALTER COLUMN updated_at DROP NOT NULL").await?;
        Ok(())
    }
}
