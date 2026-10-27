use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Reviews::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Reviews::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(Reviews::NewsletterId).string().not_null())
                    .col(ColumnDef::new(Reviews::SectionId).string().not_null())
                    .col(ColumnDef::new(Reviews::ReviewerName).string().not_null())
                    .col(ColumnDef::new(Reviews::Note).text().null())
                    .col(ColumnDef::new(Reviews::Base).json_binary().not_null())
                    .col(ColumnDef::new(Reviews::Proposed).json_binary().null())
                    .col(ColumnDef::new(Reviews::Reply).text().null())
                    .col(ColumnDef::new(Reviews::Status).string_len(16).not_null().default("open"))
                    .col(ColumnDef::new(Reviews::Revision).integer().not_null().default(0))
                    .col(ColumnDef::new(Reviews::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Reviews::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Reviews::SubmittedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Reviews::ExpiresAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reviews_newsletter_id")
                            .from(Reviews::Table, Reviews::NewsletterId)
                            .to(Newsletters::Table, Newsletters::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_newsletter_id")
                    .table(Reviews::Table)
                    .col(Reviews::NewsletterId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Reviews::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    Token,
    NewsletterId,
    SectionId,
    ReviewerName,
    Note,
    Base,
    Proposed,
    Reply,
    Status,
    Revision,
    CreatedAt,
    UpdatedAt,
    SubmittedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    Id,
}
