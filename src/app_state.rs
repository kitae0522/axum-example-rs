use axum::extract::FromRef;
use sqlx::SqlitePool;
use chrono_tz::Tz;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: SqlitePool,
    // pub timezone: Tz,
}