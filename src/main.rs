use std::env;
use sea_orm::Database;
mod auth;
mod handlers;
mod htmx;
mod models;
mod router;
mod routers;
mod services;
mod state;
mod views;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localtest.me".to_string());
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let is_produciton = match env::var("ENVIRONMENT") {
        Ok(environment) => environment == "production",
        Err(_) => false,
    };

    let app = router::create_service(db, &domain, &port, jwt_secret, is_produciton);

    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("listening on http://{address}");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
