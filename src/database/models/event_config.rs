use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EventConfig {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub event_type: String,
    pub is_enabled: bool,
}
