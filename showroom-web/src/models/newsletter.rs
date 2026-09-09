use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "newsletters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub publication_id: String,
    pub title: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub content: Json,
    pub rendered: Option<String>,
    pub sent_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
