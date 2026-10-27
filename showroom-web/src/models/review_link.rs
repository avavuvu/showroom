use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "review_links")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub review_id: String,
    pub token: String,
    pub reviewer_name: String,
    pub proposed: Option<Json>,
    pub reply: Option<String>,
    pub revision: i32,
    pub opened_at: Option<DateTimeWithTimeZone>,
    pub sent_at: Option<DateTimeWithTimeZone>,
    pub applied_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::review::Entity",
        from = "Column::ReviewId",
        to = "super::review::Column::Id",
        on_delete = "Cascade"
    )]
    Review,
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub enum Progress {
    NotOpened,
    Opened,
    Editing,
    Sent,
    EditedSinceSent,
}

impl Model {
    pub fn progress(&self) -> Progress {
        match (self.opened_at, &self.proposed, self.sent_at) {
            (_, _, Some(sent)) if self.updated_at > sent => Progress::EditedSinceSent,
            (_, _, Some(_)) => Progress::Sent,
            (_, Some(_), None) => Progress::Editing,
            (Some(_), None, None) => Progress::Opened,
            (None, None, None) => Progress::NotOpened,
        }
    }

    pub fn has_returned(&self) -> bool {
        self.sent_at.is_some()
    }
}
