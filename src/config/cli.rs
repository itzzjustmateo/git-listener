use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "git-listener", version, about = "A multi-provider Git webhook to Discord bot")]
pub struct Cli {
    #[arg(short, long, env = "GL_CONFIG")]
    pub config: Option<PathBuf>,

    #[arg(short, long, env = "GL_DATABASE__URL")]
    pub database_url: Option<String>,

    #[arg(short, long, env = "GL_DISCORD__TOKEN")]
    pub discord_token: Option<String>,

    #[arg(long = "app-id", env = "GL_DISCORD__APPLICATION_ID")]
    pub application_id: Option<u64>,

    #[arg(short = 'w', long, env = "GL_WEBHOOK__BIND_PORT")]
    pub webhook_port: Option<u16>,

    #[arg(short = 'a', long, env = "GL_API__BIND_PORT")]
    pub api_port: Option<u16>,

    #[arg(short, long, env = "GL_LOGGING__LEVEL")]
    pub log_level: Option<String>,
}
