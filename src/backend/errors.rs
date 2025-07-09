use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tokio_postgres::error::SqlState;
use tracing::{error, warn};

/// Custom error type for backend operations.
///
/// This enum centralizes error handling for database, authentication,
/// and other server-side issues.
///
/// Errors should must be in fluent message
#[cfg(feature = "server")]
#[derive(Debug, Error)]
pub enum BackendError {
    #[error("{0}")]
    AuthError(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("std-err.internal")]
    InternalError,
    #[error("{0}.not-found")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("login.required")]
    LoginRequired,
    #[error("duplicate")]
    UniqueConstraintViolation,
    #[error("frm-email.duplicate")]
    DuplicateUser,
}

// Implement `IntoResponse` for `BackendError` to convert it into an Axum response.
// This allows you to return `Result<T, BackendError>` directly from handlers.
impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            BackendError::AuthError(msg) => {
                error!("Authentication Error: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            BackendError::ValidationError(msg) => {
                error!("Validation Error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            BackendError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "unexpected".to_string())
            }
            BackendError::NotFound(msg) => {
                error!("Not Found Error: {}", msg);
                (StatusCode::NOT_FOUND, msg.to_string())
            }
            BackendError::UniqueConstraintViolation => {
                (StatusCode::BAD_REQUEST, "duplicate".to_string())
            }
            BackendError::DuplicateUser => (StatusCode::BAD_REQUEST, "duplicate user".to_string()),
            BackendError::Unauthorized => {
                error!("Unauthorized access attempt.");
                (StatusCode::UNAUTHORIZED, "unauthorized".to_string())
            }
            BackendError::LoginRequired => {
                warn!("Login required.");
                (StatusCode::UNAUTHORIZED, "unauthorized.login".to_string())
            }
            BackendError::Forbidden => {
                error!("Forbidden access attempt.");
                (StatusCode::FORBIDDEN, "forbidden".to_string())
            }
        };

        // For production, you might want to generalize internal errors
        // and only expose specific messages for client-facing issues.
        // For debugging, keep more details.
        (status, error_message).into_response()
    }
}

impl From<argon2::password_hash::Error> for BackendError {
    fn from(value: argon2::password_hash::Error) -> Self {
        match value {
            argon2::password_hash::Error::Password => {
                BackendError::ValidationError("frm-password.invalid".into())
            }
            _ => {
                error!("argon2 {:?}", value);
                BackendError::InternalError
            }
        }
    }
}

impl From<tokio_postgres::Error> for BackendError {
    fn from(error: tokio_postgres::Error) -> Self {
        match error.as_db_error().map(|s| (s.code(), s.constraint())) {
            Some((&SqlState::UNIQUE_VIOLATION, Some("app_user_email_key"))) => {
                warn!(
                    "UNIQUE_VIOLATION: try to create an new account with an already existing email"
                );
                BackendError::DuplicateUser
            }
            Some((&SqlState::UNIQUE_VIOLATION, Some(constraint))) => {
                error!("UNIQUE_VIOLATION: {}", constraint);
                BackendError::UniqueConstraintViolation
            }
            _ => {
                error!("Database Error: {:?}", error);
                BackendError::InternalError
            }
        }
    }
}

impl From<deadpool_postgres::PoolError> for BackendError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        error!("Database Error: {:?}", error);
        BackendError::InternalError
    }
}
