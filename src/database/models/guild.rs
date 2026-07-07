use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Guild {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub preferred_locale: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
