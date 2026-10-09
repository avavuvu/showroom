use std::{env, process::ExitCode};

use sea_orm::Database;
use showroom_web::{
    services::account::{self, NewAccount},
    state::Urls,
};

const PASSWORD_ALPHABET: [char; 55] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '2', '3', '4', '5', '6', '7', '9',
];
const PASSWORD_LENGTH: usize = 16;

fn database_url() -> Option<String> {
    let url = env::var("DATABASE_URL").ok()?;
    if cfg!(debug_assertions) || url.contains("sslmode") {
        return Some(url);
    }
    let separator = if url.contains('?') { '&' } else { '?' };
    Some(format!("{url}{separator}sslmode=require"))
}

#[tokio::main]
async fn main() -> ExitCode {
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().skip(1).collect();
    let [email, handle] = args.as_slice() else {
        eprintln!("Usage: create_user <email> <handle>");
        return ExitCode::FAILURE;
    };

    let Some(database_url) = database_url() else {
        eprintln!("DATABASE_URL must be set");
        return ExitCode::FAILURE;
    };

    let db = match Database::connect(&database_url).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Could not connect to the database: {e}");
            return ExitCode::FAILURE;
        }
    };

    let password = nanoid::nanoid!(PASSWORD_LENGTH, &PASSWORD_ALPHABET);
    let new_account = NewAccount { email: email.trim().to_string(), handle: handle.trim().to_string(), password: password.clone() };

    let user = match account::create(&db, &new_account).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("The account was not created: {e}");
            return ExitCode::FAILURE;
        }
    };

    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let main_domain = env::var("MAIN_DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let urls = Urls::new(domain, port, main_domain);
    let slug = new_account.handle.to_lowercase();

    println!();
    println!("Log in:    {}/login", urls.base());
    println!("Email:     {}", user.email);
    println!("Password:  {password}");
    println!();

    ExitCode::SUCCESS
}
