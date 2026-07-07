use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct WebhookEvent {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub provider: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub processed: bool,
    pub error: Option<String>,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}
