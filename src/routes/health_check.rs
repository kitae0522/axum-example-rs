use axum::Json;
use serde::{Deserialize, Serialize};

use crate::utilities::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    message: String
}

pub async fn health_check() -> Result<Json<HealthCheck>, AppError> {
    Ok(Json(HealthCheck{message: "pong".to_owned()}))
}