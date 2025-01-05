use dotenvy::dotenv;
use sqlx::SqlitePool;
use chrono_tz::Tz;

use axum_example_rs::{
    run_server,
    app_state::AppState
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("missing environment variable `DATABASE_URL`")
        .to_owned();

    let app_timezone = std::env::var("APP_TIMEZONE")
        .unwrap_or_else(|_| "UTC".to_string());

    app_timezone.parse::<Tz>().unwrap_or_else(|_| {
        eprintln!("invalid timezone in APP_TIMEZONE, falling back to UTC.");
        chrono_tz::UTC
    });
    
    let db_pool = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let state = AppState { db_pool };
    run_server(state).await;
}