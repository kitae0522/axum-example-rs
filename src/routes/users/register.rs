use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{queries, utilities::error::AppError};
use super::{RequestRegister, ResponseRegister};

pub async fn register(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<RequestRegister>
) -> Result<Json<ResponseRegister>, AppError> {

    if payload.password != payload.password_confirm {
        let error = AppError::new(StatusCode::BAD_REQUEST, "incorrect confirm password");
        return Err(error)
    }

    queries::user::create_user(
        &db_pool,
        payload.email.clone(),
        payload.password.clone()
    ).await;

    Ok(Json(ResponseRegister {}))
}