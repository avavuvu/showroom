use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comments::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Comments::NewsletterId).string().not_null())
                    .col(ColumnDef::new(Comments::SubscriberToken).string().null())
                    .col(ColumnDef::new(Comments::UserId).string().null())
                    .col(ColumnDef::new(Comments::Name).string().null())
                    .col(ColumnDef::new(Comments::Email).string().null())
                    .col(ColumnDef::new(Comments::Content).json_binary().not_null())
                    .col(ColumnDef::new(Comments::Status).string_len(16).not_null())
                    .col(ColumnDef::new(Comments::Source).string_len(16).not_null().default("web"))
                    .col(ColumnDef::new(Comments::SectionId).string().null())
                    .col(ColumnDef::new(Comments::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Comments::ExpiresAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Comments::RemovedAt).timestamp_with_time_zone().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_newsletter_id")
                            .from(Comments::Table, Comments::NewsletterId)
                            .to(Newsletters::Table, Newsletters::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_user_id")
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE comments
                    ADD CONSTRAINT comments_status_known CHECK (status IN ('unverified', 'published', 'removed')),
                    ADD CONSTRAINT comments_unverified_expires CHECK ((status = 'unverified') = (expires_at IS NOT NULL)),
                    ADD CONSTRAINT comments_removed_at CHECK ((status = 'removed') = (removed_at IS NOT NULL)),
                    ADD CONSTRAINT comments_unverified_has_email CHECK (status <> 'unverified' OR email IS NOT NULL)",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_newsletter_id_created_at")
                    .table(Comments::Table)
                    .col(Comments::NewsletterId)
                    .col(Comments::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .add_column(ColumnDef::new(Publications::CommentsEnabled).boolean().not_null().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Publications::Table)
                    .drop_column(Publications::CommentsEnabled)
                    .to_owned(),
            )
            .await?;

        manager.drop_table(Table::drop().table(Comments::Table).if_exists().to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    NewsletterId,
    SubscriberToken,
    UserId,
    Name,
    Email,
    Content,
    Status,
    Source,
    SectionId,
    CreatedAt,
    ExpiresAt,
    RemovedAt,
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    CommentsEnabled,
}
