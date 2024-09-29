use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error")]
    AuthError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("SBOM parsing error: {0}")]
    SBOMParsingError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError => (StatusCode::UNAUTHORIZED, "Authentication error"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::SBOMParsingError(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}