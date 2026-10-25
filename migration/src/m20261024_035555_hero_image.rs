use sea_orm_migration::{prelude::*};
use sea_orm_migration::sea_orm::ConnectionTrait;
use serde_json::Value;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .add_column(ColumnDef::new(Newsletters::HeroImageId).text())
                    .to_owned(),
                )
                .await?;

            let db = manager.get_connection();
            let backend = manager.get_database_backend();

            let select = Query::select()
                .columns([Newsletters::Id, Newsletters::Content])
                .from(Newsletters::Table)
                .to_owned();

            for row in db.query_all(backend.build(&select)).await? {
                let id: String = row.try_get("", "id")?;
                let content: Value = row.try_get("", "content")?;

                if let Some(public_id) = first_public_id(&content) {
                    let update = Query::update()
                        .table(Newsletters::Table)
                        .value(Newsletters::HeroImageId, public_id)
                        .and_where(Expr::col(Newsletters::Id).eq(id))
                        .to_owned();
                    db.execute(backend.build(&update)).await?;
                }
            }

            Ok(())
        }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Newsletters::Table)
                    .drop_column(Newsletters::HeroImageId)
                    .to_owned(),
            )
            .await
    }
}

fn first_public_id(node: &Value) -> Option<String> {
    if node["type"] == "image" {
        return node["attrs"]["publicId"].as_str().map(str::to_owned);
    }
    node["content"].as_array()?.iter().find_map(first_public_id)
}

#[derive(DeriveIden)]
enum Newsletters {
    Table,
    HeroImageId,
    Id,
    Content
}
