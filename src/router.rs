use axum::{
    Router,
    routing::{
        get, post, put, patch, delete
    }
};

use crate::{
    app_state::AppState,
    routes::{
        health_check::health_check,
        users::{
            login::login,
            register::register
        }
    }
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/ping", get(health_check))
        .route("/api/users/login", post(login))
        .route("/api/users/register", post(register))
        .with_state(state)
}