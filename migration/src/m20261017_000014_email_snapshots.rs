use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().table(Publications::Table).drop_column(Publications::Greeting).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .add_column(ColumnDef::new(Newsletters::EmailSubject).text().null())
                    .add_column(ColumnDef::new(Newsletters::EmailHtml).text().null())
                    .add_column(ColumnDef::new(Newsletters::EmailText).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(Index::create().name("idx_publications_owner_id").table(Publications::Table).col(Publications::OwnerId).to_owned())
            .await?;

        let connection = manager.get_connection();
        connection
            .execute_unprepared(
                "DELETE FROM subscribers AS duplicate USING subscribers AS kept \
                 WHERE duplicate.publication_id = kept.publication_id \
                 AND lower(trim(duplicate.email)) = lower(trim(kept.email)) \
                 AND (duplicate.is_confirmed, kept.created_at, duplicate.token) < (kept.is_confirmed, duplicate.created_at, kept.token)",
            )
            .await?;
        connection.execute_unprepared("UPDATE subscribers SET email = lower(trim(email)) WHERE email <> lower(trim(email))").await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_publications_owner_id").table(Publications::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .drop_column(Newsletters::EmailSubject)
                    .drop_column(Newsletters::EmailHtml)
                    .drop_column(Newsletters::EmailText)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .add_column(ColumnDef::new(Publications::Greeting).text().not_null().default("Hi {{name}},"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    OwnerId,
    Greeting,
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    EmailSubject,
    EmailHtml,
    EmailText,
}
