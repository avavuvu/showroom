use std::env;
use aws_config::{BehaviorVersion, Region};
use sea_orm::Database;
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

async fn main_in_dev() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let email_domain = env::var("EMAIL_DOMAIN").unwrap_or_else(|_| "showroom.you".to_string());
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::from_static("ap-southeast-2"))
        .load()
        .await;
    let ses = aws_sdk_sesv2::Client::new(&aws_config);

    let app = router::create_service(db, ses, &domain, &port, &email_domain, jwt_secret);

    let address = format!("localtest.me:{port}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("listening on http://{address}");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn main_in_prod() {
    todo!("write production main")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    #[cfg(debug_assertions)]
    main_in_dev().await;

     #[cfg(not(debug_assertions))]
    main_in_prod().await;
}
