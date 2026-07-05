use std::env;
use aws_config::{BehaviorVersion, Region};
use sea_orm::{Database, DatabaseConnection};
mod auth;
mod mailer;
mod handlers;
mod htmx;
mod models;
mod router;
mod routers;
mod renderer;
mod services;
mod state;
mod views;

#[derive(Clone)]
struct AppEnv {
    db: DatabaseConnection,
    ses: aws_sdk_sesv2::Client,
    port: String,
    domain: String,
    main_domain: String,
    jwt_secret: String,
}

async fn setup() -> AppEnv {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let main_domain = env::var("MAIN_DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::from_static("ap-southeast-2"))
        .load()
        .await;
    let ses = aws_sdk_sesv2::Client::new(&aws_config);

    AppEnv { db, ses, port, domain, main_domain, jwt_secret }
}

async fn server(env: AppEnv) {
    let app = router::create_service(
        env.db,
        env.ses,
        &env.domain,
        &env.port,
        &env.main_domain,
        env.jwt_secret,
    );

    let address = format!("0.0.0.0:{}", env.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    #[cfg(feature = "local")]
    println!("listening on http://localtest.me:{}", env.port);
    #[cfg(not(feature = "local"))]
    println!("listening on http://{address}");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(feature = "local")]
#[tokio::main]
async fn main() {
    let env = setup().await;
    dioxus_devtools::serve_subsecond_with_args(
        env,
        |e| async { server(e).await },
    ).await;
}

#[cfg(not(feature = "local"))]
#[tokio::main]
async fn main() {
    let env = setup().await;
    server(env).await;
}
