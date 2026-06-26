use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(email: &str, user_id: &str) -> Self {
        Self {
            sub: email.to_string(),
            user_id: user_id.to_string(),
            exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        }
    }
}

pub fn generate(secret: &[u8], claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn validate(secret: &[u8], token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default())
        .map(|data| data.claims)
}
