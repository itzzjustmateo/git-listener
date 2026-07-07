use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::event_config::EventConfig;
use crate::errors::AppResult;

pub async fn get_event_configs(
    pool: &PgPool,
    repository_id: Uuid,
) -> AppResult<Vec<EventConfig>> {
    let results = sqlx::query_as::<_, EventConfig>(
        "SELECT * FROM event_configs WHERE repository_id = $1",
    )
    .bind(repository_id)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn upsert_event_config(
    pool: &PgPool,
    repository_id: Uuid,
    event_type: &str,
    is_enabled: bool,
) -> AppResult<EventConfig> {
    let result = sqlx::query_as::<_, EventConfig>(
        r#"
        INSERT INTO event_configs (repository_id, event_type, is_enabled)
        VALUES ($1, $2, $3)
        ON CONFLICT (repository_id, event_type) DO UPDATE SET
            is_enabled = EXCLUDED.is_enabled
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(event_type)
    .bind(is_enabled)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
