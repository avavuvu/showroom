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
                    .table(Subscribers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subscribers::Token).string().not_null().primary_key())
                    .col(ColumnDef::new(Subscribers::UserId).string().not_null())
                    .col(ColumnDef::new(Subscribers::Name).string().null())
                    .col(ColumnDef::new(Subscribers::Email).string().not_null())
                    .col(ColumnDef::new(Subscribers::IsConfirmed).boolean().not_null().default(false))
                    .col(ColumnDef::new(Subscribers::CreatedAt).timestamp_with_time_zone().not_null())
                    .index(Index::create().unique().col(Subscribers::UserId).col(Subscribers::Email))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_subscribers_user_id")
                    .from(Subscribers::Table, Subscribers::UserId)
                    .to(Users::Table, Users::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_subscribers_user_id")
                    .table(Subscribers::Table)
                    .to_owned(),
            )
            .await?;

        manager.drop_table(Table::drop().table(Subscribers::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Subscribers {
    Table,
    Token,
    UserId,
    Name,
    Email,
    IsConfirmed,
    CreatedAt,
}
