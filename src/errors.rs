use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("BSON error: {0}")]
    Bson(#[from] bson::ser::Error),
    
    #[error("DNS error: {0}")]
    Dns(String),
    
    #[error("AI processing error: {0}")]
    Ai(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::Serialization(err) => {
                tracing::error!("Serialization error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
            }
            AppError::Bson(err) => {
                tracing::error!("BSON error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Data processing error")
            }
            AppError::Dns(err) => {
                tracing::error!("DNS error: {}", err);
                (StatusCode::SERVICE_UNAVAILABLE, "DNS resolution failed")
            }
            AppError::Ai(err) => {
                tracing::error!("AI error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "AI processing failed")
            }
            AppError::Auth(err) => {
                tracing::warn!("Auth error: {}", err);
                (StatusCode::UNAUTHORIZED, "Authentication failed")
            }
            AppError::Validation(err) => {
                tracing::warn!("Validation error: {}", err);
                (StatusCode::BAD_REQUEST, &err)
            }
            AppError::NotFound(err) => {
                tracing::warn!("Not found: {}", err);
                (StatusCode::NOT_FOUND, &err)
            }
            AppError::BadRequest(err) => {
                tracing::warn!("Bad request: {}", err);
                (StatusCode::BAD_REQUEST, &err)
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;