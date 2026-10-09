use std::fmt;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, TransactionTrait};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    models::{publication::{self, Entity as Publication}, user::{self, Entity as User}},
    renderer::greeting,
};


fn alphanumeric(value: &str) -> Result<(), validator::ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        let mut e = validator::ValidationError::new("alphanumeric");
        e.message = Some("Handle can only contain letters and numbers".into());
        Err(e)
    }
}

#[derive(Deserialize, Validate)]
pub struct NewAccount {
    #[validate(email(message = "Enter a valid email address"))]
    pub email: String,
    #[validate(
        custom(function = "alphanumeric", message = "Handle can only contain letters and numbers"),
        length(min = 3, max = 20, message = "Handle must be between 3 and 20 characters")
    )]
    pub handle: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

pub enum AccountError {
    Invalid(ValidationErrors),
    Fields(Vec<(&'static str, String)>),
    Hash(String),
    Database(DbErr),
}

impl From<DbErr> for AccountError {
    fn from(error: DbErr) -> Self {
        AccountError::Database(error)
    }
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::Invalid(errors) => {
                let messages: Vec<String> = errors
                    .field_errors()
                    .into_iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            error.message.as_ref().map(|message| message.to_string()).unwrap_or_else(|| format!("Invalid {field}"))
                        })
                    })
                    .collect();
                f.write_str(&messages.join("; "))
            }
            AccountError::Fields(fields) => {
                f.write_str(&fields.iter().map(|(_, message)| message.as_str()).collect::<Vec<_>>().join("; "))
            }
            AccountError::Hash(error) => write!(f, "The password could not be hashed: {error}"),
            AccountError::Database(error) => write!(f, "Database error: {error}"),
        }
    }
}


pub async fn create(db: &DatabaseConnection, account: &NewAccount) -> Result<user::Model, AccountError> {
    account.validate().map_err(AccountError::Invalid)?;

    let slug = account.handle.to_lowercase();
    if let Err(message) = publication::validate_slug(&slug) {
        return Err(AccountError::Fields(vec![("handle", message.to_string())]));
    }

    let email_taken = User::find().filter(user::Column::Email.eq(&account.email)).one(db).await?.is_some();
    let handle_taken = Publication::find().filter(publication::Column::Slug.eq(&slug)).one(db).await?.is_some();

    if email_taken || handle_taken {
        let mut fields = Vec::new();
        if email_taken {
            fields.push(("email", "An account with this email already exists".to_string()));
        }
        if handle_taken {
            fields.push(("handle", "This handle is already taken".to_string()));
        }
        return Err(AccountError::Fields(fields));
    }

    let new_user = user::new(&account.email, &account.password).map_err(|e| AccountError::Hash(e.to_string()))?;

    let now = chrono::Utc::now().fixed_offset();
    let default_room = publication::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        owner_id: new_user.id.clone(),
        slug: Set(slug.clone()),
        name: Set(format!("{slug}'s room")),
        description: Set(None),
        theme: Set(None),
        greeting: Set(greeting::DEFAULT.to_string()),
        image: Set(publication::random_picture()),
        banner: Set(None),
        pictures: Set(Default::default()),
        is_default: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = db
        .transaction::<_, user::Model, DbErr>(|txn| {
            Box::pin(async move {
                let user = new_user.insert(txn).await?;
                default_room.insert(txn).await?;
                Ok(user)
            })
        })
        .await
        .map_err(|e| match e {
            sea_orm::TransactionError::Connection(e) | sea_orm::TransactionError::Transaction(e) => AccountError::Database(e),
        })?;

    Ok(user)
}
