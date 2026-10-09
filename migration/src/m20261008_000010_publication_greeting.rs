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
                    .add_column(ColumnDef::new(Publications::Greeting).text().not_null().default("Hi {{name}},"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .drop_column(Publications::Greeting)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    Greeting,
}
