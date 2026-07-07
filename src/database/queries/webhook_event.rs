use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::webhook_event::WebhookEvent;
use crate::errors::AppResult;

pub async fn insert_webhook_event(
    pool: &PgPool,
    repository_id: Uuid,
    provider: &str,
    event_type: &str,
    payload: &serde_json::Value,
) -> AppResult<WebhookEvent> {
    let result = sqlx::query_as::<_, WebhookEvent>(
        r#"
        INSERT INTO webhook_events (repository_id, provider, event_type, payload)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(provider)
    .bind(event_type)
    .bind(payload)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn mark_processed(
    pool: &PgPool,
    id: Uuid,
    error: Option<&str>,
) -> AppResult<WebhookEvent> {
    let result = sqlx::query_as::<_, WebhookEvent>(
        r#"
        UPDATE webhook_events
        SET processed = true, processed_at = NOW(), error = $2
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(error)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
