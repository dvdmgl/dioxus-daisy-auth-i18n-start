use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use validator::Validate;

#[cfg(feature = "server")]
use crate::backend::auth::SessionWrapper;

/// Struct for user login payload (from frontend to backend).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Validate))]
pub struct Credentials {
    #[cfg_attr(feature = "server", validate(email))]
    pub email: String,
    #[cfg_attr(feature = "server", validate(length(min = 8, max = 16)))]
    pub password: String,
    pub next: Option<String>,
}

/// Struct for user registration payload (from frontend to backend).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(Validate))]
pub struct RegisterPayload {
    #[cfg_attr(feature = "server", validate(email))]
    pub email: String,
    // #[cfg_attr(feature = "server", validate(must_match(other = "password2")))]
    pub password: String,
    // pub password2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(Validate))]
pub struct CheckEmail {
    #[cfg_attr(feature = "server", validate(email))]
    pub email: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(postgres_types::ToSql, postgres_types::FromSql)
)]
#[cfg_attr(feature = "server", postgres(name = "user_role"))]
pub enum UserRole {
    #[cfg_attr(feature = "server", postgres(name = "admin"))]
    Admin,
    #[cfg_attr(feature = "server", postgres(name = "staff"))]
    Staff,
    #[cfg_attr(feature = "server", postgres(name = "user"))]
    User,
    #[cfg_attr(feature = "server", postgres(name = "guest"))]
    Guest,
    #[cfg_attr(feature = "server", postgres(name = "naughty"))]
    Naughty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub c_at: DateTime<Utc>,
    pub u_at: DateTime<Utc>,
    pub role: UserRole,
}

#[server(SubmitCreateUser)]
pub async fn submit_create_user(payload: RegisterPayload) -> Result<Option<User>, ServerFnError> {
    let auth: axum::Extension<crate::backend::BackendState> = extract().await?;
    let client = auth.0.db.get().await?;
    payload.validate()?;

    use crate::backend::user::create_user;
    let entry = create_user(&client, payload.email, payload.password).await?;

    Ok(Some(entry))
}

#[server(LoginUser)]
pub async fn login_user(payload: Credentials) -> Result<Option<User>, ServerFnError> {
    use axum_login::AuthnBackend;
    let mut session: SessionWrapper = extract().await?;
    if let Some(user) = session.session.backend.authenticate(payload).await? {
        session.session.login(&user).await?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

#[server(LogoutUser)]
pub async fn logout_user() -> Result<(), ServerFnError> {
    let mut session: SessionWrapper = extract().await?;
    session.session.logout().await?;
    Ok(())
}

#[server(CheckUserIsFree)]
pub async fn check_user_is_free(payload: CheckEmail) -> Result<Option<bool>, ServerFnError> {
    let auth: axum::Extension<crate::backend::BackendState> = extract().await?;
    let client = auth.0.db.get().await?;
    payload.validate()?;

    use crate::backend::user::check_email;
    let entry = check_email(&client, payload.email).await?;

    Ok(Some(entry))
}

#[server(GetUserSession)]
pub async fn get_user_session() -> Result<Option<User>, ServerFnError> {
    let session: SessionWrapper = extract().await?;
    match session.session.user {
        Some(user) => Ok(Some(user)),
        None => Ok(None),
    }
}

#[server(UserSessionLogout)]
pub async fn user_session_logout() -> Result<(), ServerFnError> {
    let mut session: SessionWrapper = extract().await?;
    session.session.logout().await?;
    Ok(())
}
