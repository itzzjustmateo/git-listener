use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::filter::Filter;
use crate::errors::AppResult;

pub async fn get_filters(pool: &PgPool, repository_id: Uuid) -> AppResult<Vec<Filter>> {
    let results = sqlx::query_as::<_, Filter>(
        "SELECT * FROM filters WHERE repository_id = $1 AND is_active = true ORDER BY created_at",
    )
    .bind(repository_id)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn create_filter(
    pool: &PgPool,
    repository_id: Uuid,
    filter_type: &str,
    filter_value: &str,
    filter_action: &str,
) -> AppResult<Filter> {
    let result = sqlx::query_as::<_, Filter>(
        r#"
        INSERT INTO filters (repository_id, filter_type, filter_value, filter_action)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(filter_type)
    .bind(filter_value)
    .bind(filter_action)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn delete_filter(pool: &PgPool, id: Uuid) -> AppResult<Option<Filter>> {
    let result = sqlx::query_as::<_, Filter>(
        "UPDATE filters SET is_active = false WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
