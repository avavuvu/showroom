use sea_orm::entity::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum Status {
    #[sea_orm(string_value = "queued")]
    Queued,
    #[sea_orm(string_value = "sending")]
    Sending,
    #[sea_orm(string_value = "sent")]
    Sent,
    #[sea_orm(string_value = "failed")]
    Failed,
    #[sea_orm(string_value = "unknown")]
    Unknown,
    #[sea_orm(string_value = "skipped")]
    Skipped,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "newsletter_deliveries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub newsletter_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub email: String,
    pub subscriber_token: String,
    pub status: Status,
    pub attempts: i32,
    pub next_attempt_at: Option<DateTimeWithTimeZone>,
    pub claimed_at: Option<DateTimeWithTimeZone>,
    pub message_id: Option<String>,
    pub error: Option<String>,
    pub sent_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
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
