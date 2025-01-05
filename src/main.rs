use dotenvy::dotenv;
use sqlx::SqlitePool;

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

    let db_pool = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let state = AppState { db_pool };
    run_server(state).await;
}