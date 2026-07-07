use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MentionConfig {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub mention_type: String,
    pub mention_id: Option<i64>,
    pub event_type: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
