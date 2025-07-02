mod authz;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier as _Argon2Verifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_login::{AuthUser, AuthnBackend, UserId};
use tracing::instrument;

use crate::shared::user::{Credentials, User};

use super::{BackendState, errors::BackendError};

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.skey.as_bytes()
    }
}

#[axum::async_trait]
impl AuthnBackend for BackendState {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let client = self.db.get().await?;

        let stmt = client
            .prepare_typed_cached(
                "SELECT id, c_at, m_at, skey, email, role, password_hash \n
         FROM app_user \n
         WHERE email = $1",
                &[tokio_postgres::types::Type::TEXT],
            )
            .await?;

        let resp = client.query_opt(&stmt, &[&creds.email]).await;
        match resp {
            Ok(None) => Err(BackendError::NotFound("user".into())),
            Ok(Some(row)) => {
                verify_password(&creds.password, row.get::<_, &str>(6))?;
                Ok(Some(User::from(row)))
            }
            Err(err) => Err(err.into()),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let client = self.db.get().await?;

        let stmt = client
            .prepare_typed_cached(
                "SELECT id, c_at, m_at, skey, email, role \n
         FROM app_user \n
         WHERE id = $1",
                &[tokio_postgres::types::Type::INT8],
            )
            .await?;
        let user = client.query_opt(&stmt, &[&user_id]).await?.map(User::from);
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<BackendState>;

#[derive(Debug, Clone)]
pub struct SessionWrapper {
    pub session: AuthSession,
}

#[derive(Debug)]
pub struct StateError;

impl std::error::Error for StateError {}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(internal) state error")
    }
}

impl axum::response::IntoResponse for StateError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "(internal) state error",
        )
            .into_response()
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for SessionWrapper
where
    S: Send + Sync,
{
    type Rejection = StateError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = AuthSession::from_request_parts(parts, state).await;
        match session {
            Ok(session) => Ok(Self { session }),
            Err(_) => Err(StateError),
        }
    }
}

/// Hashes a plain-text password using Argon2.
///
/// This function takes a password string and returns its Argon2 hash.
#[instrument(level = "debug", skip(password))]
pub fn hash_password(password: &str) -> Result<String, BackendError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| BackendError::AuthError(format!("Failed to hash password: {e}")))?
        .to_string())
}

/// Verifies a plain-text password against a stored Argon2 hash.
///
/// This function is used during the login process to check if the provided password
/// matches the stored hash.
#[instrument(level = "debug", skip(password, password_hash))]
pub fn verify_password(password: &str, password_hash: &str) -> Result<(), BackendError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| BackendError::AuthError(format!("Failed to parse password hash: {e}")))?;

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash)?)
}
