use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::config::DatabaseConfig;
use crate::errors::AppResult;

pub async fn create_pool(config: &DatabaseConfig) -> AppResult<PgPool> {
    let opts: PgConnectOptions = config.url.parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(
            config.connect_timeout_seconds,
        ))
        .connect_with(opts)
        .await?;

    tracing::info!("database pool established");

    Ok(pool)
}
