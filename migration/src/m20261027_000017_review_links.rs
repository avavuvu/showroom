use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Reviews::Table).if_exists().to_owned()).await?;

        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .col(ColumnDef::new(Reviews::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Reviews::NewsletterId).string().not_null())
                    .col(ColumnDef::new(Reviews::SectionId).string().null())
                    .col(ColumnDef::new(Reviews::AfterSectionId).string().null())
                    .col(ColumnDef::new(Reviews::Note).text().null())
                    .col(ColumnDef::new(Reviews::Base).json_binary().not_null())
                    .col(ColumnDef::new(Reviews::CreatedAt).timestamp_with_time_zone().not_null())
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
            .create_index(Index::create().name("idx_reviews_newsletter_id").table(Reviews::Table).col(Reviews::NewsletterId).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReviewLinks::Table)
                    .col(ColumnDef::new(ReviewLinks::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ReviewLinks::ReviewId).string().not_null())
                    .col(ColumnDef::new(ReviewLinks::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(ReviewLinks::ReviewerName).string().not_null())
                    .col(ColumnDef::new(ReviewLinks::Proposed).json_binary().null())
                    .col(ColumnDef::new(ReviewLinks::Reply).text().null())
                    .col(ColumnDef::new(ReviewLinks::Revision).integer().not_null().default(0))
                    .col(ColumnDef::new(ReviewLinks::OpenedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(ReviewLinks::SentAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(ReviewLinks::AppliedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(ReviewLinks::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(ReviewLinks::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_links_review_id")
                            .from(ReviewLinks::Table, ReviewLinks::ReviewId)
                            .to(Reviews::Table, Reviews::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(Index::create().name("idx_review_links_review_id").table(ReviewLinks::Table).col(ReviewLinks::ReviewId).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ReviewLinks::Table).if_exists().to_owned()).await?;
        manager.drop_table(Table::drop().table(Reviews::Table).if_exists().to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    NewsletterId,
    SectionId,
    AfterSectionId,
    Note,
    Base,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum ReviewLinks {
    Table,
    Id,
    ReviewId,
    Token,
    ReviewerName,
    Proposed,
    Reply,
    Revision,
    OpenedAt,
    SentAt,
    AppliedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    Id,
}
