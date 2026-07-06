use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use chrono::Utc;
use fake::faker::internet::en::{SafeEmail, Username};
use fake::faker::lorem::en::Paragraphs;
use fake::faker::company::en::BsNoun;
use fake::Fake;
use nanoid::nanoid;
use sea_orm::{ActiveValue::Set, Database, EntityTrait};
use serde_json::json;
use showroom_rs::models::{newsletter, user};
use std::env;

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn tiptap_doc(paragraphs: Vec<String>) -> serde_json::Value {
    json!({
        "type": "doc",
        "content": paragraphs.iter().map(|p| json!({
            "type": "paragraph",
            "content": [{ "type": "text", "text": p }]
        })).collect::<Vec<_>>()
    })
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.expect("Failed to connect to database");

    println!("Seeding database...");

    let password = hash_password("password");

    for _ in 0..3 {
        let user_id = nanoid!(14);
        let username: String = Username().fake();
        let email: String = SafeEmail().fake();
        let now = Utc::now().fixed_offset();

        user::Entity::insert(user::ActiveModel {
            id: Set(user_id.clone()),
            handle: Set(username.clone()),
            email: Set(email.clone()),
            password: Set(password.clone()),
            created_at: Set(now),
            updated_at: Set(Some(now)),
        })
        .exec(&db)
        .await
        .expect("Failed to insert user");

        println!("  user: {username} <{email}>");

        for _ in 0..5 {
            let word1: String = BsNoun().fake();
            let word2: String = BsNoun().fake();
            let title = format!("On {word1} and {word2}");
            let paragraphs: Vec<String> = Paragraphs(2..4).fake();

            newsletter::Entity::insert(newsletter::ActiveModel {
                id: Set(nanoid!(14)),
                user_id: Set(user_id.clone()),
                title: Set(title.clone()),
                slug: Set(slugify(&title)),
                subtitle: Set(None),
                content: Set(tiptap_doc(paragraphs)),
                rendered: Set(None),
                sent_at: Set(None),
                created_at: Set(now),
                updated_at: Set(now),
            })
            .exec(&db)
            .await
            .expect("Failed to insert newsletter");

            println!("    newsletter: {title}");
        }
    }

    println!("\nDone.");
}
