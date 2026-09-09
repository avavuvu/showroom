use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subscribers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token: String,
    pub publication_id: String,
    pub name: Option<String>,
    pub email: String,
    pub is_confirmed: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::publication::Entity",
        from = "Column::PublicationId",
        to = "super::publication::Column::Id"
    )]
    Publication,
}

impl Related<super::publication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publication.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
