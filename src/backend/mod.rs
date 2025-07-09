pub mod auth;
pub mod errors;
mod otlp;
pub mod user;

use axum::{Extension, extract::FromRef};
use axum_extra::extract::cookie::{Key, SameSite};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time},
};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use dioxus::{fullstack::*, prelude::*};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use tokio_postgres::NoTls;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::shared::user::{UserPermission, UserRole};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub postgres: PostgresConfig,
    pub otlp_endpoint: String,
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
    pub groups: HashMap<UserRole, HashSet<UserPermission>>,
}

impl BackendState {
    async fn new(db: Pool) -> Self {
        let client = db.get().await.expect("failed to get client");

        let stmt = client
            .prepare_typed_cached("SELECT role, permission FROM app_groups_permissions", &[])
            .await
            .expect("failed to prepare statement");

        let rows = client
            .query(&stmt, &[])
            .await
            .expect("failed to make query");
        let mut groups: HashMap<UserRole, HashSet<UserPermission>> = HashMap::new();
        for r in rows {
            let (role, permission): (UserRole, UserPermission) = (r.get(0), r.get(1));
            groups.entry(role).or_default().insert(permission);
        }
        Self {
            db,
            key: Key::generate(),
            groups,
        }
    }
}

/// Allows extracting the `Key` from `AppState`.
impl FromRef<BackendState> for Key {
    fn from_ref(state: &BackendState) -> Self {
        state.key.clone()
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, errors::BackendError> {
        dotenvy::dotenv().map_err(|_e| errors::BackendError::InternalError)?;
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
        Ok(Self {
            postgres,
            otlp_endpoint: std::env::var("OTLP_ENDPOINT").expect("OTLP_ENDPOINT"),
        })
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
        .expect("failed to create database pool no tls connection")
        .max_size(20)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("failed create database pool");

    let state = BackendState::new(pool).await;
    let provider = otlp::init_tracer(&config.otlp_endpoint);

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    let auth_layer = AuthManagerLayerBuilder::new(state.clone(), session_layer).build();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), crate::app::App)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(Extension(state))
        .layer(auth_layer)
        .into_make_service();

    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    provider
        .shutdown()
        .expect("Failed to close tracer provider");
}
