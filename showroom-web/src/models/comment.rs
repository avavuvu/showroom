use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum Status {
    #[sea_orm(string_value = "unverified")]
    Unverified,
    #[sea_orm(string_value = "published")]
    Published,
    #[sea_orm(string_value = "removed")]
    Removed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum Source {
    #[sea_orm(string_value = "web")]
    Web,
    #[sea_orm(string_value = "email")]
    Email,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub newsletter_id: String,
    pub subscriber_token: Option<String>,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub content: Json,
    pub status: Status,
    pub source: Source,
    pub section_id: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub removed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::newsletter::Entity",
        from = "Column::NewsletterId",
        to = "super::newsletter::Column::Id",
        on_delete = "Cascade"
    )]
    Newsletter,
}

impl Related<super::newsletter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Newsletter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
