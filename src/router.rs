use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::storage::{
    user_storage::UserStorage,
    // post_storage::PostStorage
};
use crate::services::{
    user_service::UserService,
    // post_service::PostService
};
use crate::controllers::{
    user_ctrl::UserController,
    // post_ctrl::PostController
};

pub fn create_user_router(db_pool: SqlitePool) -> Router {
    let user_storage = UserStorage::new(db_pool.clone());
    let user_service = UserService::new(user_storage);
    let user_ctrl = UserController::new(user_service);

    let mut router = Router::new()
        .route("/user/login", post(user_ctrl.login_user))
        .route("/user/register", post(user_ctrl.create_user))
        .route("/user/{id}", get(user_ctrl.get_user));
    
    router
}
