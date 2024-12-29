use std::sync::Arc;
use sqlx::SqlitePool;

use crate::router::create_user_router;
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

mod controllers;
mod services;
mod storage;
mod models;
mod router;

#[tokio::main]
async fn main() {
    let db_pool = SqlitePool::connect("todos.db").await.unwrap();
    let app = create_user_router(db_pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server Running");
    axum::serve(listener, app).await.unwrap();
}