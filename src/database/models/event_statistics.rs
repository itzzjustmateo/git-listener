use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EventStatistic {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub event_type: String,
    pub count: i64,
    pub last_event_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
