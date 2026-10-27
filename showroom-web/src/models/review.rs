use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub newsletter_id: String,
    pub section_id: Option<String>,
    pub after_section_id: Option<String>,
    pub note: Option<String>,
    pub base: Json,
    pub created_at: DateTimeWithTimeZone,
    pub expires_at: DateTimeWithTimeZone,
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
    #[sea_orm(has_many = "super::review_link::Entity")]
    Links,
}

impl Related<super::newsletter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Newsletter.def()
    }
}

impl Related<super::review_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Links.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn blocks(&self) -> Vec<crate::document::Block> {
        crate::document::Document::from_stored(&self.base).content
    }

    pub fn label(&self) -> String {
        use crate::document::{Block, inline_text};
        if self.section_id.is_none() {
            return "Whole newsletter".to_string();
        }
        match self.blocks().first() {
            Some(Block::Heading { content, .. }) if !inline_text(content).trim().is_empty() => inline_text(content),
            _ => "Untitled section".to_string(),
        }
    }
}

#[boutique::ts]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummary {
    pub section_id: Option<String>,
    pub people: usize,
    pub returned: usize,
}
