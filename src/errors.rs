use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(mongodb::error::Error),
    Serialization(serde_json::Error),
    Bson(bson::ser::Error),
    Redis(String),
    Dns(String),
    Config(String),
    Validation(String),
    NotFound(String),
    Unauthorized(String),
    RateLimited(String),
    Internal(String),
    Io(std::io::Error),
    TrustDns(trust_dns_client::error::ClientError),
    Bb8(bb8::RunError<bb8_redis::redis::RedisError>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "Database error: {}", err),
            AppError::Serialization(err) => write!(f, "Serialization error: {}", err),
            AppError::Bson(err) => write!(f, "BSON error: {}", err),
            AppError::Redis(err) => write!(f, "Redis error: {}", err),
            AppError::Dns(err) => write!(f, "DNS error: {}", err),
            AppError::Config(err) => write!(f, "Configuration error: {}", err),
            AppError::Validation(err) => write!(f, "Validation error: {}", err),
            AppError::NotFound(err) => write!(f, "Not found: {}", err),
            AppError::Unauthorized(err) => write!(f, "Unauthorized: {}", err),
            AppError::RateLimited(err) => write!(f, "Rate limited: {}", err),
            AppError::Internal(err) => write!(f, "Internal error: {}", err),
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::TrustDns(err) => write!(f, "Trust DNS error: {}", err),
            AppError::Bb8(err) => write!(f, "Connection pool error: {}", err),
        }
    }
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
                (StatusCode::BAD_REQUEST, "Invalid data format")
            }
            AppError::Bson(err) => {
                tracing::error!("BSON error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Data processing error")
            }
            AppError::Redis(err) => {
                tracing::error!("Redis error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Cache error occurred")
            }
            AppError::Dns(err) => {
                tracing::error!("DNS error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "DNS resolution failed")
            }
            AppError::Config(err) => {
                tracing::error!("Config error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            AppError::Validation(ref err) => {
                tracing::warn!("Validation error: {}", err);
                (StatusCode::BAD_REQUEST, err.as_str())
            }
            AppError::NotFound(ref err) => {
                tracing::info!("Not found: {}", err);
                (StatusCode::NOT_FOUND, err.as_str())
            }
            AppError::Unauthorized(_) => {
                (StatusCode::UNAUTHORIZED, "Authentication failed")
            }
            AppError::RateLimited(ref err) => {
                tracing::warn!("Rate limited: {}", err);
                (StatusCode::TOO_MANY_REQUESTS, err.as_str())
            }
            AppError::Internal(ref err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err.as_str())
            }
            AppError::Io(err) => {
                tracing::error!("IO error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "IO error occurred")
            }
            AppError::TrustDns(err) => {
                tracing::error!("Trust DNS error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "DNS client error")
            }
            AppError::Bb8(err) => {
                tracing::error!("Connection pool error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Connection pool error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// From implementations for error conversion
impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err)
    }
}

impl From<bson::ser::Error> for AppError {
    fn from(err: bson::ser::Error) -> Self {
        AppError::Bson(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<trust_dns_client::error::ClientError> for AppError {
    fn from(err: trust_dns_client::error::ClientError) -> Self {
        AppError::TrustDns(err)
    }
}

impl From<bb8::RunError<bb8_redis::redis::RedisError>> for AppError {
    fn from(err: bb8::RunError<bb8_redis::redis::RedisError>) -> Self {
        AppError::Bb8(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;