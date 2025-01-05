use axum::{extract::State, Json};
use sqlx::SqlitePool;

use crate::{
    queries,
    utilities::{
        error::AppError,
        password::verify_login_password
    }
};
use super::{RequestLogin, ResponseLogin};

pub async fn login(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<RequestLogin>
) -> Result<Json<ResponseLogin>, AppError> {
    match login_impl(&db_pool, &payload).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err)
    }
}

async fn login_impl(db_pool: &SqlitePool, payload: &RequestLogin) -> Result<ResponseLogin, AppError> {
    let user = queries::user::find_user_by_email(db_pool, payload.email.clone()).await.unwrap();
    verify_login_password(&user.hash_password, &payload.password.clone())?;
    Ok(ResponseLogin { id: user.id, email: user.email })
}