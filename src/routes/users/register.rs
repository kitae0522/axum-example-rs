use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::{
    queries,
    utilities::{
        error::AppError,
        password::verify_register_password
    }
};
use super::{RequestRegister, ResponseRegister};

pub async fn register(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<RequestRegister>
) -> Result<Json<ResponseRegister>, AppError> {
    match register_impl(&db_pool, &payload).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err)
    }
}

async fn register_impl(db_pool: &SqlitePool, payload: &RequestRegister) -> Result<ResponseRegister, AppError> {
    verify_register_password(&payload.password, &payload.password_confirm)?;

    // TODO: Encrypt Password with Salt
    let sample_salt = "pepper".to_owned();
    let hash_password = payload.password.clone();
    queries::user::create_user(db_pool, payload.email.clone(), hash_password, sample_salt).await;

    Ok(ResponseRegister {})
}