use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub webhook: WebhookConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub run_migrations: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
    pub intents: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookConfig {
    pub bind_address: String,
    pub bind_port: u16,
    pub max_body_size: usize,
    pub rate_limit_burst: u32,
    pub rate_limit_period_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub bind_port: u16,
    pub api_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let cli = crate::config::Cli::parse();

        let config_path = cli
            .config
            .clone()
            .unwrap_or_else(|| PathBuf::from("config"));

        let env = std::env::var("GL_APP__ENVIRONMENT")
            .unwrap_or_else(|_| "development".into());

        let config_file = config_path.join("default.toml");
        let env_file = config_path.join(format!("{env}.toml"));

        let mut builder = Config::builder()
            .add_source(File::from(config_file).required(false))
            .add_source(File::from(env_file).required(false))
            .add_source(
                Environment::with_prefix("GL")
                    .separator("__")
                    .try_parsing(true),
            );

        if let Some(db_url) = cli.database_url {
            builder = builder.set_override("database.url", db_url)?;
        }
        if let Some(token) = cli.discord_token {
            builder = builder.set_override("discord.token", token)?;
        }
        if let Some(app_id) = cli.application_id {
            builder = builder.set_override("discord.application_id", app_id as u64)?;
        }
        if let Some(port) = cli.webhook_port {
            builder = builder.set_override("webhook.bind_port", port)?;
        }
        if let Some(port) = cli.api_port {
            builder = builder.set_override("api.bind_port", port)?;
        }
        if let Some(level) = cli.log_level {
            builder = builder.set_override("logging.level", level)?;
        }

        let settings: Settings = builder.build()?.try_deserialize()?;

        if settings.discord.token.is_empty() {
            return Err(ConfigError::Message(
                "discord.token is required — set GL_DISCORD__TOKEN or --discord-token".into(),
            ));
        }

        Ok(settings)
    }
}
