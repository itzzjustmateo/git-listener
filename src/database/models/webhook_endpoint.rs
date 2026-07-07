use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct WebhookEndpoint {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub endpoint_token: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
