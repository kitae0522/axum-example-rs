use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};

use crate::models::user_model::{CreateUser, LoginUser};
use crate::services::user_service::UserService;

pub struct UserController {
    service: UserService
}

impl UserController {
    pub fn new(service: UserService) -> UserController {
        UserController { service }
    }

    pub async fn login_user(&self, Json(payload): Json<LoginUser>) -> impl IntoResponse {
        let user = self.service.login_user(&payload);
        (StatusCode::OK, Json(user));
    }

    pub async fn create_user(&self, Json(payload): Json<CreateUser>) -> impl IntoResponse {
        let user = self.service.create_user(&payload);
        (StatusCode::OK, Json(user));
    }

    pub async fn get_user(&self, Path(id): Path<String>) -> impl IntoResponse {
        "User data"
    }
}