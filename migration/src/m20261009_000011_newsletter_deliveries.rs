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
                    .add_column(ColumnDef::new(Newsletters::PublishedAt).timestamp_with_time_zone().null())
                    .add_column(ColumnDef::new(Newsletters::SendStartedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Newsletters::Table)
                    .value(Newsletters::PublishedAt, Expr::col(Newsletters::SentAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .drop_column(Newsletters::SentAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(NewsletterDeliveries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(NewsletterDeliveries::NewsletterId).string().not_null())
                    .col(ColumnDef::new(NewsletterDeliveries::Email).string().not_null())
                    .col(ColumnDef::new(NewsletterDeliveries::SubscriberToken).string().not_null())
                    .col(ColumnDef::new(NewsletterDeliveries::Status).string_len(16).not_null().default("queued"))
                    .col(ColumnDef::new(NewsletterDeliveries::Attempts).integer().not_null().default(0))
                    .col(ColumnDef::new(NewsletterDeliveries::NextAttemptAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(NewsletterDeliveries::ClaimedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(NewsletterDeliveries::MessageId).string().null())
                    .col(ColumnDef::new(NewsletterDeliveries::Error).text().null())
                    .col(ColumnDef::new(NewsletterDeliveries::SentAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(NewsletterDeliveries::CreatedAt).timestamp_with_time_zone().not_null())
                    .primary_key(
                        Index::create()
                            .col(NewsletterDeliveries::NewsletterId)
                            .col(NewsletterDeliveries::Email),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_newsletter_deliveries_newsletter_id")
                            .from(NewsletterDeliveries::Table, NewsletterDeliveries::NewsletterId)
                            .to(Newsletters::Table, Newsletters::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_newsletter_deliveries_status")
                    .table(NewsletterDeliveries::Table)
                    .col(NewsletterDeliveries::Status)
                    .col(NewsletterDeliveries::NextAttemptAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NewsletterDeliveries::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .add_column(ColumnDef::new(Newsletters::SentAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Newsletters::Table)
                    .value(Newsletters::SentAt, Expr::col(Newsletters::PublishedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .drop_column(Newsletters::PublishedAt)
                    .drop_column(Newsletters::SendStartedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NewsletterDeliveries {
    Table,
    NewsletterId,
    Email,
    SubscriberToken,
    Status,
    Attempts,
    NextAttemptAt,
    ClaimedAt,
    MessageId,
    Error,
    SentAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    Id,
    SentAt,
    PublishedAt,
    SendStartedAt,
}
