use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub guild_id: i64,
    pub actor_id: i64,
    pub action: String,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
