use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .add_column(ColumnDef::new(Newsletters::Revision).integer().not_null().default(0))
                    .drop_column(Newsletters::Rendered)
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();
        conn.execute_unprepared("UPDATE newsletters SET slug = id WHERE sent_at IS NULL").await?;
        conn.execute_unprepared(
            "UPDATE newsletters AS n SET slug = n.slug || '-' || n.id \
             FROM ( \
                 SELECT id, row_number() OVER (PARTITION BY publication_id, slug ORDER BY sent_at, id) AS position \
                 FROM newsletters \
             ) AS d \
             WHERE n.id = d.id AND d.position > 1",
        )
        .await?;
        conn.execute_unprepared("UPDATE newsletters SET slug = id WHERE slug = ''").await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_newsletters_publication_slug")
                    .table(Newsletters::Table)
                    .col(Newsletters::PublicationId)
                    .col(Newsletters::Slug)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_newsletters_publication_slug").table(Newsletters::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .drop_column(Newsletters::Revision)
                    .add_column(ColumnDef::new(Newsletters::Rendered).text().null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    PublicationId,
    Slug,
    Revision,
    Rendered,
}
