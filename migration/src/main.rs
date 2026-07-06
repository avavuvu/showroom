use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    unsafe {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            std::env::set_var("DATABASE_URL", ensure_ssl(&url));
        }
    }
    cli::run_cli(migration::Migrator).await;
}

fn ensure_ssl(url: &str) -> String {
    if url.contains("sslmode") {
        url.to_string()
    } else if url.contains('?') {
        format!("{}&sslmode=require", url)
    } else {
        format!("{}?sslmode=require", url)
    }
}
