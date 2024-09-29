use thiserror::Error;
use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFoundError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred"),
            AppError::AuthError(_) => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFoundError(_) => (StatusCode::NOT_FOUND, "Resource not found"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}