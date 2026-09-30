use boutique::{AuthStore, NewSession};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, TransactionTrait,
};

use crate::models::{session, user};

#[derive(Clone)]
pub struct Store {
    db: DatabaseConnection,
}

impl Store {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl AuthStore<user::Model> for Store {
    type Error = DbErr;

    async fn find_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find().filter(user::Column::Email.eq(email)).one(&self.db).await
    }

    async fn find_user_by_id(&self, id: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(&self.db).await
    }

    async fn create_session(&self, new: NewSession) -> Result<(), DbErr> {
        session::ActiveModel {
            id: Set(new.token_hash),
            user_id: Set(new.user_id),
            created_at: Set(new.created_at.into()),
            expires_at: Set(new.expires_at.into()),
        }
        .insert(&self.db)
        .await?;
        Ok(())
    }

    async fn find_session_user(&self, token_hash: &str, now: DateTime<Utc>) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .inner_join(session::Entity)
            .filter(session::Column::Id.eq(token_hash))
            .filter(session::Column::ExpiresAt.gt(now))
            .one(&self.db)
            .await
    }

    async fn delete_session(&self, token_hash: &str) -> Result<(), DbErr> {
        session::Entity::delete_by_id(token_hash).exec(&self.db).await?;
        Ok(())
    }

    async fn delete_user_sessions(&self, user_id: &str) -> Result<(), DbErr> {
        session::Entity::delete_many().filter(session::Column::UserId.eq(user_id)).exec(&self.db).await?;
        Ok(())
    }

    async fn delete_expired_sessions(&self, now: DateTime<Utc>) -> Result<(), DbErr> {
        session::Entity::delete_many().filter(session::Column::ExpiresAt.lte(now)).exec(&self.db).await?;
        Ok(())
    }

    async fn reset_password(&self, user: user::Model, password_hash: String) -> Result<user::Model, DbErr> {
        let transaction = self.db.begin().await?;

        session::Entity::delete_many().filter(session::Column::UserId.eq(&user.id)).exec(&transaction).await?;

        let mut active = user.into_active_model();
        active.password = Set(password_hash);
        let user = active.update(&transaction).await?;

        transaction.commit().await?;
        Ok(user)
    }
}
