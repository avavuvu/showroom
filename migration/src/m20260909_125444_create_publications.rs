use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::PgExpr;

use super::m20250101_000001_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Publications::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Publications::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Publications::OwnerId).string().not_null())
                    .col(ColumnDef::new(Publications::Slug).string_len(40).not_null().unique_key())
                    .col(ColumnDef::new(Publications::Name).string().not_null())
                    .col(ColumnDef::new(Publications::Description).text().null())
                    .col(ColumnDef::new(Publications::Theme).json_binary().null())
                    .col(ColumnDef::new(Publications::IsDefault).boolean().not_null().default(false))
                    .col(ColumnDef::new(Publications::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Publications::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_publications_owner_id")
                            .from(Publications::Table, Publications::OwnerId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // one default "room" per existing user, reusing the user id as the publication id
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Publications::Table)
                    .columns([
                        Publications::Id,
                        Publications::OwnerId,
                        Publications::Slug,
                        Publications::Name,
                        Publications::IsDefault,
                        Publications::CreatedAt,
                        Publications::UpdatedAt,
                    ])
                    .select_from(
                        Query::select()
                            .from(Users::Table)
                            .column(Users::Id)
                            .column(Users::Id)
                            .column(Users::Handle)
                            .expr(Expr::col(Users::Handle).concat("'s room"))
                            .expr(Expr::value(true))
                            .column(Users::CreatedAt)
                            .expr(Func::coalesce([
                                Expr::col(Users::UpdatedAt).into(),
                                Expr::col(Users::CreatedAt).into(),
                            ]))
                            .to_owned(),
                    )
                    .map_err(|e| DbErr::Migration(e.to_string()))?
                    .to_owned(),
            )
            .await?;

        // sea-query has no partial index support, so this one stays raw
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE UNIQUE INDEX idx_publications_owner_default ON publications (owner_id) WHERE is_default",
            )
            .await?;

        move_owner(manager, Newsletters::Table, Newsletters::UserId, Newsletters::PublicationId, "fk_newsletters_user_id", "fk_newsletters_publication_id").await?;
        move_owner(manager, Subscribers::Table, Subscribers::UserId, Subscribers::PublicationId, "fk_subscribers_user_id", "fk_subscribers_publication_id").await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_subscribers_publication_id_email")
                    .table(Subscribers::Table)
                    .col(Subscribers::PublicationId)
                    .col(Subscribers::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Handle)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Handle).string_len(20).null())
                    .to_owned(),
            )
            .await?;
        manager
            .exec_stmt(
                Query::update()
                    .table(Users::Table)
                    .value(
                        Users::Handle,
                        SimpleExpr::SubQuery(
                            None,
                            Box::new(
                                Query::select()
                                    .from(Publications::Table)
                                    .column(Publications::Slug)
                                    .and_where(Expr::col((Publications::Table, Publications::OwnerId)).equals((Users::Table, Users::Id)))
                                    .and_where(Expr::col(Publications::IsDefault).eq(true))
                                    .to_owned()
                                    .into_sub_query_statement(),
                            ),
                        ),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_subscribers_publication_id_email")
                    .table(Subscribers::Table)
                    .to_owned(),
            )
            .await?;

        restore_owner(manager, Newsletters::Table, Newsletters::UserId, Newsletters::PublicationId, "fk_newsletters_user_id", "fk_newsletters_publication_id").await?;
        restore_owner(manager, Subscribers::Table, Subscribers::UserId, Subscribers::PublicationId, "fk_subscribers_user_id", "fk_subscribers_publication_id").await?;

        manager.drop_table(Table::drop().table(Publications::Table).to_owned()).await
    }
}

async fn move_owner<T, U, P>(
    manager: &SchemaManager<'_>,
    table: T,
    user_col: U,
    publication_col: P,
    old_fk: &str,
    new_fk: &str,
) -> Result<(), DbErr>
where
    T: IntoTableRef + IntoIden + Copy + 'static,
    U: IntoIden + Copy + 'static,
    P: IntoIden + Copy + 'static,
{
    manager
        .alter_table(
            Table::alter()
                .table(table)
                .add_column(ColumnDef::new(publication_col).string().null())
                .to_owned(),
        )
        .await?;
    manager
        .exec_stmt(
            Query::update()
                .table(table)
                .value(publication_col, Expr::col(user_col))
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table(table)
                .modify_column(ColumnDef::new(publication_col).string().not_null())
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name(new_fk)
                .from(table, publication_col)
                .to(Publications::Table, Publications::Id)
                .to_owned(),
        )
        .await?;
    manager
        .drop_foreign_key(ForeignKey::drop().name(old_fk).table(table).to_owned())
        .await?;
    manager
        .alter_table(Table::alter().table(table).drop_column(user_col).to_owned())
        .await
}

async fn restore_owner<T, U, P>(
    manager: &SchemaManager<'_>,
    table: T,
    user_col: U,
    publication_col: P,
    old_fk: &str,
    new_fk: &str,
) -> Result<(), DbErr>
where
    T: IntoTableRef + IntoIden + Copy + 'static,
    U: IntoIden + Copy + 'static,
    P: IntoIden + Copy + 'static,
{
    manager
        .alter_table(
            Table::alter()
                .table(table)
                .add_column(ColumnDef::new(user_col).string().null())
                .to_owned(),
        )
        .await?;
    manager
        .exec_stmt(
            Query::update()
                .table(table)
                .value(
                    user_col,
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(
                            Query::select()
                                .from(Publications::Table)
                                .column(Publications::OwnerId)
                                .and_where(Expr::col((Publications::Table, Publications::Id)).equals((table, publication_col)))
                                .to_owned()
                                .into_sub_query_statement(),
                        ),
                    ),
                )
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table(table)
                .modify_column(ColumnDef::new(user_col).string().not_null())
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name(old_fk)
                .from(table, user_col)
                .to(Users::Table, Users::Id)
                .to_owned(),
        )
        .await?;
    manager
        .drop_foreign_key(ForeignKey::drop().name(new_fk).table(table).to_owned())
        .await?;
    manager
        .alter_table(Table::alter().table(table).drop_column(publication_col).to_owned())
        .await
}

#[derive(DeriveIden)]
enum Publications {
    Table,
    Id,
    OwnerId,
    Slug,
    Name,
    Description,
    Theme,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden, Clone, Copy)]
enum Newsletters {
    Table,
    UserId,
    PublicationId,
}

#[derive(DeriveIden, Clone, Copy)]
enum Subscribers {
    Table,
    UserId,
    PublicationId,
    Email,
}
