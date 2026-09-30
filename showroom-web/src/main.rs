use std::env;
use aws_config::{BehaviorVersion, Region};
use sea_orm::{Database, DatabaseConnection};
mod auth;
mod mailer;
mod handlers;
mod models;
mod router;
mod routers;
mod renderer;
mod services;
mod state;
mod theme;
mod views;

use boutique::cloudinary::Cloudinary;

#[derive(Clone)]
struct AppEnv {
    db: DatabaseConnection,
    ses: aws_sdk_sesv2::Client,
    cloudinary: Cloudinary,
    port: String,
    domain: String,
    main_domain: String,
    secret: String,
}

// this is used in prod, cargo misses that
#[allow(unused)]
fn ensure_ssl(url: &str) -> String {
    if url.contains("sslmode") {
        url.to_string()
    } else if url.contains('?') {
        format!("{}&sslmode=require", url)
    } else {
        format!("{}?sslmode=require", url)
    }
}

async fn setup() -> AppEnv {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    #[cfg(not(debug_assertions))]
    let database_url = ensure_ssl(&database_url);
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let main_domain = env::var("MAIN_DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::from_static("ap-southeast-2"))
        .load()
        .await;
    let ses = aws_sdk_sesv2::Client::new(&aws_config);

    let cloudinary = Cloudinary::from_env().expect("CLOUDINARY_URL must be set");

    AppEnv { db, ses, cloudinary, port, domain, main_domain, secret }
}

async fn server(env: AppEnv) {
    let state = state::AppState::new(
        env.db,
        env.ses,
        env.cloudinary,
        state::Urls::new(&env.domain, &env.port, &env.main_domain),
        env.secret,
    );

    let (server, routes) = router::create_server(&state);

    #[cfg(feature = "local")]
    println!("listening on http://localtest.me:{}", env.port);
    #[cfg(not(feature = "local"))]
    println!("listening on http://0.0.0.0:{}", env.port);

    server.serve(routes, &env.port).await;
}

#[tokio::main]
async fn main() {
    let env = setup().await;
    boutique::run(env, server).await;
}
