use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{queries, utilities::error::AppError};
use super::{RequestLogin, ResponseLogin};

pub async fn login(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<RequestLogin>
) -> Result<Json<ResponseLogin>, AppError> {

    let user = queries::user::find_user_by_email(&db_pool, payload.email.clone()).await;

    if user.password != payload.password {
        let error = AppError::new(StatusCode::UNAUTHORIZED, "wrong password");
        return Err(error);
    }
    
    let response = ResponseLogin {
        id: user.id,
        email: user.email
    };

    Ok(Json(response))
}