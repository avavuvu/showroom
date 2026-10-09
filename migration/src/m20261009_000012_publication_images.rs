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
                    .add_column(ColumnDef::new(Publications::Image).string().null())
                    .add_column(ColumnDef::new(Publications::Banner).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Publications::Table)
                    .value(Publications::Image, Expr::cust("'default/' || floor(random() * 10)::int"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .modify_column(ColumnDef::new(Publications::Image).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .drop_column(Publications::Image)
                    .drop_column(Publications::Banner)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    Image,
    Banner,
}
