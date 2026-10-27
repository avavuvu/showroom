use boutique::cloudinary::Cloudinary;
use sea_orm::{QuerySelect, entity::prelude::*};

use crate::state::Urls;

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
    pub revision: i32,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub send_started_at: Option<DateTimeWithTimeZone>,
    pub email_subject: Option<String>,
    pub email_html: Option<String>,
    pub email_text: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub hero_image_id: Option<String>
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

pub const RESERVED_SLUGS: [&str; 8] = ["api", "feed", "subscribe", "confirm", "unsubscribe", "about", "contact", "archive"];

impl Entity {
    pub fn find_summaries() -> Select<Entity> {
        Entity::find()
            .select_only()
            .columns([
                Column::Id,
                Column::PublicationId,
                Column::Title,
                Column::Slug,
                Column::Subtitle,
                Column::Revision,
                Column::PublishedAt,
                Column::SendStartedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
                Column::HeroImageId,
            ])
            .column_as(Expr::cust("'null'::jsonb"), Column::Content)
            .column_as(Expr::cust("NULL::text"), Column::EmailSubject)
            .column_as(Expr::cust("NULL::text"), Column::EmailHtml)
            .column_as(Expr::cust("NULL::text"), Column::EmailText)
    }
}

impl Model {
    pub fn display_title(&self) -> &str {
        let title = self.title.trim();
        if title.is_empty() { "Untitled" } else { title }
    }
}
