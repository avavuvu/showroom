use boutique::AuthUser;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::publication::Entity")]
    Publication,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::publication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publication.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AuthUser for Model {
    type Store = crate::auth::Store;

    fn id(&self) -> &str {
        &self.id
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn password_hash(&self) -> &str {
        &self.password
    }
}

pub fn new(email: &str, plain_password: &str) -> Result<ActiveModel, boutique::argon2::password_hash::Error> {
    Ok(ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(email.to_string()),
        password: Set(boutique::password::hash(plain_password)?),
        created_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    })
}
