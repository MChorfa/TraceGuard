use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use log::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("AppError: {:?}", self);
        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json("Database error occurred"),
            AppError::AuthError(_) => HttpResponse::Unauthorized().json("Authentication failed"),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(msg),
            AppError::NotFoundError(msg) => HttpResponse::NotFound().json(msg),
            AppError::InternalServerError => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}