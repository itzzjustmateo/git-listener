use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EmbedConfig {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub color: Option<i32>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub use_thread: bool,
    pub thread_name: Option<String>,
    pub use_forum: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
