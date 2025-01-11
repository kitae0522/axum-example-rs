use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use sqlx::SqlitePool;

use crate::{
    queries,
    utilities::{
        error::AppError,
        password::verify_register_password
    }
};
use super::{RequestRegister, ResponseRegister, RequestCreateProfile, ResponseCreateProfile};

pub async fn register(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<RequestRegister>
) -> Result<Json<ResponseRegister>, AppError> {
    match register_impl(&db_pool, &payload).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err)
    }
}

pub async fn create_profile(
    State(db_pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<RequestCreateProfile>
) -> Result<Json<ResponseCreateProfile>, AppError> {
    match create_profile_impl(&db_pool, &id, &payload).await {
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

async fn create_profile_impl(db_pool: &SqlitePool, id: &String, payload: &RequestCreateProfile) -> Result<ResponseCreateProfile, AppError> {
    let user = queries::user::get_user_by_id(db_pool, id.clone()).await
        .map_err(|_| AppError { code: StatusCode::NOT_FOUND, message: "user not found".to_owned() });

    let profile = queries::user::update_user_profile(
        db_pool,
        id.clone(),
        payload.name.clone(),
        payload.handle.clone(),
        payload.profile_pic_url.clone(),
        payload.bio.clone()
    ).await.unwrap();

    Ok(ResponseCreateProfile { 
        id: profile.id,
        email: profile.email,
        name: profile.name.unwrap(),
        handle: profile.handle.unwrap(),
        profile_pic_url: profile.profile_pic_url.unwrap(),
        bio: profile.bio.unwrap()
    })
}