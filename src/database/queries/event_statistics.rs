use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::event_statistics::EventStatistic;
use crate::errors::AppResult;

pub async fn increment_event_count(
    pool: &PgPool,
    repository_id: Uuid,
    event_type: &str,
) -> AppResult<EventStatistic> {
    let result = sqlx::query_as::<_, EventStatistic>(
        r#"
        INSERT INTO event_statistics (repository_id, event_type, count, last_event_at)
        VALUES ($1, $2, 1, NOW())
        ON CONFLICT (repository_id, event_type) DO UPDATE SET
            count = event_statistics.count + 1,
            last_event_at = NOW()
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(event_type)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_statistics(
    pool: &PgPool,
    repository_id: Uuid,
) -> AppResult<Vec<EventStatistic>> {
    let results = sqlx::query_as::<_, EventStatistic>(
        "SELECT * FROM event_statistics WHERE repository_id = $1 ORDER BY count DESC",
    )
    .bind(repository_id)
    .fetch_all(pool)
    .await?;

    Ok(results)
}
