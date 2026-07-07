use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Filter {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub filter_type: String,
    pub filter_value: String,
    pub filter_action: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
