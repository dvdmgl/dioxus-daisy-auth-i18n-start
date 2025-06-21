pub mod auth;
mod errors;
pub mod user;

use axum::{Extension, extract::FromRef};
use axum_extra::extract::cookie::{Key, SameSite};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time},
};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use dioxus::{fullstack::*, prelude::*};
use errors::BackendError;
use serde::Deserialize;
use tokio_postgres::NoTls;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub postgres: PostgresConfig,
}
#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Debug, Clone)]
pub struct BackendState {
    /// The database connection pool.
    pub db: Pool,
    /// A key used for signing and verifying cookies.
    pub key: Key,
}

/// Allows extracting the `Key` from `AppState`.
impl FromRef<BackendState> for Key {
    fn from_ref(state: &BackendState) -> Self {
        state.key.clone()
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, errors::BackendError> {
        dotenvy::dotenv().map_err(|_e| errors::BackendError::InternalError("failed".into()))?;
        let postgres = PostgresConfig {
            host: std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST"),
            port: std::env::var("POSTGRES_PORT")
                .expect("POSTGRES_PORT")
                .parse()
                .expect("failed to parse POSTGRES_PORT"),
            user: std::env::var("POSTGRES_USER").expect("POSTGRES_USER"),
            password: std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD"),
            db: std::env::var("POSTGRES_DB").expect("POSTGRES_DB"),
        };
        Ok(Self { postgres })
    }
}

pub async fn launch_server(_component: fn() -> Element) {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let config = AppConfig::new().expect("loaded config with success");
    let pg_config = Config {
        host: Some(config.postgres.host),
        port: Some(config.postgres.port),
        password: Some(config.postgres.password),
        dbname: Some(config.postgres.db),
        user: Some(config.postgres.user),
        manager: Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        }),
        ..Default::default()
    };

    let pool = pg_config
        .builder(NoTls)
        .map_err(|e| {
            error!("Failed to build database pool config: {}", e);
            BackendError::DbError(format!("Failed to build database pool config: {}", e))
        })
        .expect("connect with tls")
        .max_size(20)
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|e| {
            error!("Failed to build database pool: {}", e);
            BackendError::DbError(format!("Failed to build database pool: {}", e))
        })
        .expect("to create a pool");

    let state = BackendState {
        key: Key::generate(),
        db: pool,
    };

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    let auth_layer = AuthManagerLayerBuilder::new(state.clone(), session_layer).build();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), crate::app::App)
        .layer(Extension(state))
        .layer(auth_layer)
        .into_make_service();

    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
