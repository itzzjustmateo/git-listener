use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Repository {
    pub id: Uuid,
    pub guild_id: i64,
    pub provider: String,
    pub provider_repo_id: Option<String>,
    pub owner_name: String,
    pub repo_name: String,
    pub full_name: String,
    pub webhook_secret_hash: String,
    pub destination_channel_id: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
