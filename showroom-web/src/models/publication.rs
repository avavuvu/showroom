use sea_orm::{entity::prelude::*, QueryOrder};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "publications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub owner_id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub theme: Option<Json>,
    pub is_default: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    Owner,
    #[sea_orm(has_many = "super::newsletter::Entity")]
    Newsletter,
    #[sea_orm(has_many = "super::subscriber::Entity")]
    Subscriber,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<super::newsletter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Newsletter.def()
    }
}

impl Related<super::subscriber::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscriber.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn for_owner(owner_id: &str, db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find()
        .filter(Column::OwnerId.eq(owner_id))
        .order_by_desc(Column::IsDefault)
        .order_by_asc(Column::CreatedAt)
        .all(db)
        .await
}

pub const RESERVED_SLUGS: &[&str] = &[
    "about", "admin", "api", "app", "assets", "css", "icons", "images", "json", "login",
    "logout", "mail", "new", "settings", "show", "signup", "sitemap", "static", "www",
];

pub fn validate_slug(slug: &str) -> Result<(), &'static str> {
    let len = slug.chars().count();
    if !(3..=40).contains(&len) {
        return Err("Must be between 3 and 40 characters");
    }
    if !slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err("Can only contain lowercase letters, numbers and hyphens");
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        return Err("Cannot start or end with a hyphen");
    }
    if RESERVED_SLUGS.contains(&slug) {
        return Err("This name is reserved");
    }
    Ok(())
}
