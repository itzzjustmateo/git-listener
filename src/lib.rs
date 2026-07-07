pub mod api;
pub mod config;
pub mod database;
pub mod discord;
pub mod errors;
pub mod filtering;
pub mod metrics;
pub mod permissions;
pub mod providers;
pub mod rendering;
pub mod services;
pub mod telemetry;
pub mod utils;
pub mod webhooks;

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<config::Settings>,
    pub db: sqlx::PgPool,
}

impl AppState {
    pub async fn new(config: config::Settings, db: sqlx::PgPool) -> anyhow::Result<Self> {
        Ok(Self {
            config: Arc::new(config),
            db,
        })
    }
}
