use tracing::{error, info, instrument};

use crate::{
    backend::auth::hash_password,
    shared::user::{User, UserRole},
};

use super::{auth::verify_password, errors::BackendError};

impl From<tokio_postgres::Row> for User {
    #[inline]
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get(0),
            c_at: row.get(1),
            u_at: row.get(2),
            skey: row.get(3),
            email: row.get(4),
            role: row.get(5),
        }
    }
}

#[instrument(level = "info", skip(client, password))]
pub async fn create_user(
    client: &deadpool_postgres::Client,
    email: String,
    password: String,
) -> Result<User, BackendError> {
    info!("Attempting to create user with email: {}", email);
    let hashed_password = hash_password(&password)?;
    let stmt = client
        .prepare_typed_cached(
            "INSERT INTO app_user (email, password_hash, role) \n
                                            VALUES ($1, $2, $3) \n
                                            RETURNING id, c_at, m_at, skey, email, role",
            &[
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
            ],
        )
        .await?;
    // Insert the new user into the database
    let row = client
        .query_one(&stmt, &[&email, &hashed_password, &UserRole::User])
        .await
        .map_err(|e| {
            error!("Failed to insert user into DB: {e}");
            BackendError::DbError(format!("Failed to create user: {e}"))
        })?;
    let user = User::from(row);

    info!("User created successfully: {}", user.email);
    Ok(user)
}

#[instrument(level = "info", skip(client, password))]
pub async fn set_user_password(
    client: &deadpool_postgres::Client,
    user: i64,
    password: &str,
) -> Result<(), BackendError> {
    info!("Attempting to set user password: {}", &user);
    let hashed_password = hash_password(password)?;
    let stmt = client
        .prepare_typed_cached(
            "UPDATE app_user \n
            SET password_hash = $2 \n
            WHERE id = $1 \n
            RETURNING true",
            &[
                tokio_postgres::types::Type::INT8,
                tokio_postgres::types::Type::TEXT,
            ],
        )
        .await?;
    let _row = client
        .query_one(&stmt, &[&user, &hashed_password])
        .await
        .map_err(|e| {
            error!("Failed to update user password: {e}");
            BackendError::DbError(format!("Failed to update user password: {e}"))
        })?;

    info!("User updated successfully: {user}");
    Ok(())
}

#[instrument(level = "info", skip(client))]
pub async fn check_email(
    client: &deadpool_postgres::Client,
    email: String,
) -> Result<bool, BackendError> {
    info!("Attempting to check if email it's free: {email}");
    let stmt = client
        .prepare_typed_cached(
            "SELECT true FROM app_user where email = $1",
            &[tokio_postgres::types::Type::TEXT],
        )
        .await?;
    // check if the user email in db
    let row = client.query_opt(&stmt, &[&email]).await.map_err(|e| {
        error!("Failed to insert user into DB: {e}");
        BackendError::DbError(format!("Failed to create user: {e}"))
    })?;
    if row.is_some() { Ok(false) } else { Ok(true) }
}

pub async fn validate_password(
    client: &deadpool_postgres::Client,
    user: i64,
    password: &str,
) -> Result<(), BackendError> {
    let stmt = client
        .prepare_typed_cached(
            "SELECT password_hash \n
     FROM app_user \n
     WHERE id = $1",
            &[tokio_postgres::types::Type::INT8],
        )
        .await?;

    let resp = dbg!(client.query_one(&stmt, &[&user]).await)?;
    verify_password(password, resp.get::<_, &str>(0))
}
