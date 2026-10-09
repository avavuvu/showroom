use sea_orm::{FromJsonQueryResult, QueryOrder, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::{state::Urls, theme::Theme};

pub const DEFAULT_PICTURES: u8 = 10;
pub const BANNER_WIDTH: u32 = 1200;
pub const BANNER_HEIGHT: u32 = 300;
pub const EMAIL_PICTURE_SIZE: u32 = 48;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "publications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub owner_id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub theme: Option<Theme>,
    pub greeting: String,
    pub image: String,
    pub banner: Option<String>,
    pub pictures: Pictures,
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

impl Model {
    pub fn theme(&self) -> Theme {
        self.theme.unwrap_or_default()
    }

    pub fn picture(&self) -> Picture<'_> {
        Picture::parse(&self.image)
    }

    pub fn picture_url(&self, urls: &Urls) -> String {
        self.picture().url(urls)
    }

    pub fn email_picture_url(&self, urls: &Urls) -> String {
        match self.picture() {
            Picture::Default(index) => urls.asset(&format!("/avatars/{index}.png")),
            Picture::Upload(public_id) => urls.cloudinary(
                public_id,
                &format!("c_fill,g_auto,w_{size},h_{size},f_png", size = EMAIL_PICTURE_SIZE * 2),
            ),
        }
    }

    pub fn banner_url(&self, urls: &Urls) -> Option<String> {
        self.banner.as_deref().map(|public_id| {
            urls.cloudinary(public_id, &format!("c_fill,g_auto,w_{BANNER_WIDTH},h_{BANNER_HEIGHT},f_auto,q_auto"))
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Picture<'a> {
    Default(u8),
    Upload(&'a str),
}

impl<'a> Picture<'a> {
    pub fn url(self, urls: &Urls) -> String {
        match self {
            Picture::Default(index) => default_picture_path(index),
            Picture::Upload(public_id) => urls.cloudinary(public_id, "c_fill,g_auto,w_256,h_256,f_auto,q_auto"),
        }
    }

    pub fn parse(image: &'a str) -> Self {
        match image.strip_prefix("default/").and_then(|index| index.parse::<u8>().ok()) {
            Some(index) if index < DEFAULT_PICTURES => Picture::Default(index),
            Some(_) => Picture::Default(0),
            None => Picture::Upload(image),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Pictures(pub Vec<String>);

impl Pictures {
    pub fn contains(&self, public_id: &str) -> bool {
        self.0.iter().any(|picture| picture == public_id)
    }

    pub fn with_newest(mut self, public_id: String) -> Self {
        self.0.retain(|picture| *picture != public_id);
        self.0.insert(0, public_id);
        self
    }
}

pub fn default_picture_path(index: u8) -> String {
    format!("/avatars/{index}.webp")
}

pub fn default_picture(index: u8) -> String {
    format!("default/{}", index % DEFAULT_PICTURES)
}

pub fn random_picture() -> String {
    default_picture(uuid::Uuid::new_v4().as_bytes()[0])
}

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
