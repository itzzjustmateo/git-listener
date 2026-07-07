use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::webhook_endpoint::WebhookEndpoint;
use crate::errors::AppResult;

pub async fn get_endpoint_by_token(
    pool: &PgPool,
    token: &str,
) -> AppResult<Option<WebhookEndpoint>> {
    let result = sqlx::query_as::<_, WebhookEndpoint>(
        "SELECT * FROM webhook_endpoints WHERE endpoint_token = $1 AND is_active = true",
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn create_endpoint(
    pool: &PgPool,
    repository_id: Uuid,
    token: &str,
) -> AppResult<WebhookEndpoint> {
    let result = sqlx::query_as::<_, WebhookEndpoint>(
        r#"
        INSERT INTO webhook_endpoints (repository_id, endpoint_token)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn revoke_endpoint(pool: &PgPool, id: Uuid) -> AppResult<Option<WebhookEndpoint>> {
    let result = sqlx::query_as::<_, WebhookEndpoint>(
        "UPDATE webhook_endpoints SET is_active = false WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
