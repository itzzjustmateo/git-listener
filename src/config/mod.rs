mod cli;
mod settings;

pub use cli::Cli;
pub use settings::{
    ApiConfig, AppConfig, DatabaseConfig, DiscordConfig, LoggingConfig, Settings, WebhookConfig,
};

use clap::Parser;

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
