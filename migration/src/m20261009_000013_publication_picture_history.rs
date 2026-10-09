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
                    .add_column(ColumnDef::new(Publications::Pictures).json_binary().not_null().default(Expr::cust("'[]'::jsonb")))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Publications::Table)
                    .value(Publications::Pictures, Expr::cust("jsonb_build_array(image)"))
                    .and_where(Expr::col(Publications::Image).not_like("default/%"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .drop_column(Publications::Pictures)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    Image,
    Pictures,
}
