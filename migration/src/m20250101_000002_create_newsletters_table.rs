use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Newsletters::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Newsletters::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Newsletters::UserId).string().not_null())
                    .col(ColumnDef::new(Newsletters::Title).string().not_null().default("Untitled"))
                    .col(ColumnDef::new(Newsletters::Slug).string().not_null())
                    .col(ColumnDef::new(Newsletters::Subtitle).string().null())
                    .col(ColumnDef::new(Newsletters::Content).json_binary().not_null())
                    .col(ColumnDef::new(Newsletters::Sent).boolean().not_null().default(false))
                    .col(ColumnDef::new(Newsletters::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Newsletters::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_newsletters_user_id")
                    .from(Newsletters::Table, Newsletters::UserId)
                    .to(Users::Table, Users::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_newsletters_user_id")
                    .table(Newsletters::Table)
                    .to_owned(),
            )
            .await?;

        manager.drop_table(Table::drop().table(Newsletters::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    Id,
    UserId,
    Title,
    Slug,
    Subtitle,
    Content,
    Sent,
    CreatedAt,
    UpdatedAt,
}
