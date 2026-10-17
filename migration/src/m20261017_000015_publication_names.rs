use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .add_column(ColumnDef::new(Publications::NameCase).string_len(16).not_null().default("typed"))
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared("UPDATE publications SET name = slug || '\u{2019}s room'").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().table(Publications::Table).drop_column(Publications::NameCase).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    NameCase,
}
