use std::{env, time::Duration};
use aws_config::{BehaviorVersion, Region};
use sea_orm::{Database, DatabaseConnection};
mod blocks;
mod document;
mod images;
mod mailer;
mod handlers;
mod models;
mod router;
mod routers;
mod renderer;
mod services;
mod state;
mod theme;
mod components;
mod views;

use boutique::cloudinary::Cloudinary;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppEnv {
    db: DatabaseConnection,
    ses: aws_sdk_sesv2::Client,
    cloudinary: Cloudinary,
    port: String,
    secret: String,
}

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

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")))
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    #[cfg(not(debug_assertions))]
    let database_url = ensure_ssl(&database_url);
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::from_static("ap-southeast-2"))
        .load()
        .await;
    let ses = aws_sdk_sesv2::Client::new(&aws_config);

    let cloudinary = Cloudinary::from_env().expect("CLOUDINARY_URL must be set");

    AppEnv { db, ses, cloudinary, port, secret }
}

async fn server(env: AppEnv) {
    let urls = state::Urls { cloudinary_cloud: env.cloudinary.cloud_name().to_string(), ..state::Urls::from_env() };

    let state = state::AppState::new(env.db, env.ses, env.cloudinary, urls, env.secret);
    let _worker = AbortOnDrop(tokio::spawn(services::newsletter_send::supervise_worker(state.clone())));
    let _session_cleanup = AbortOnDrop(boutique::session::spawn_cleanup(state.auth.clone(), Duration::from_secs(60 * 60)));

    let (server, routes) = router::create_server(&state);

    #[cfg(feature = "local")]
    println!("listening on http://localtest.me:{}", env.port);
    #[cfg(not(feature = "local"))]
    println!("listening on http://0.0.0.0:{}", env.port);

    if let Err(error) = server.serve(routes, format!("0.0.0.0:{}", env.port)).await {
        tracing::error!(%error, "server failed");
        std::process::exit(1);
    }
}

struct AbortOnDrop(tokio::task::JoinHandle<()>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[tokio::main]
async fn main() {
    let env = setup().await;
    boutique::run(env, server).await;
}
