use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::{error, warn};

/// Custom error type for backend operations.
///
/// This enum centralizes error handling for database, authentication,
/// and other server-side issues.
#[cfg(feature = "server")]
#[derive(Debug, Error)]
pub enum BackendError {
    #[error(transparent)]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    TokioPostgresError(#[from] tokio_postgres::Error),
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("{0}.not-found")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("login.required")]
    LoginRequired,
}

// Implement `IntoResponse` for `BackendError` to convert it into an Axum response.
// This allows you to return `Result<T, BackendError>` directly from handlers.
impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            BackendError::PoolError(e) => {
                error!("Pool Error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server.internal".to_string(),
                )
            }
            BackendError::TokioPostgresError(e) => {
                error!("Database Error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server.internal".to_string(),
                )
            }
            BackendError::DbError(msg) => {
                error!("Database Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred.".to_string(),
                )
            }
            BackendError::AuthError(msg) => {
                error!("Authentication Error: {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            BackendError::ValidationError(msg) => {
                error!("Validation Error: {}", msg);
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            BackendError::InternalError(msg) => {
                error!("Internal Server Error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "unexpected".to_string())
            }
            BackendError::NotFound(msg) => {
                error!("Not Found Error: {}", msg);
                (StatusCode::NOT_FOUND, msg.to_string())
            }
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
                BackendError::InternalError("frm-password.invalid".into())
            }
        }
    }
}
