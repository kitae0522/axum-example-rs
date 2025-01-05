use crate::utilities::error::AppError;
use axum::http::StatusCode;

pub fn verify_login_password(stored_password: &str, input_password: &str) -> Result<(), AppError> {
    if stored_password != input_password {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Incorrect password"));
    }
    Ok(())
}

pub fn verify_register_password(password: &str, confirm_password: &str) -> Result<(), AppError> {
    if password != confirm_password {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Passwords do not match"));
    }
    Ok(())
}
