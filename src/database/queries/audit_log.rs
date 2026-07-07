use sqlx::PgPool;

use crate::database::models::audit_log::AuditLog;
use crate::errors::AppResult;

pub async fn create_audit_log(
    pool: &PgPool,
    guild_id: i64,
    actor_id: i64,
    action: &str,
    details: Option<&serde_json::Value>,
) -> AppResult<AuditLog> {
    let result = sqlx::query_as::<_, AuditLog>(
        r#"
        INSERT INTO audit_logs (guild_id, actor_id, action, details)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(guild_id)
    .bind(actor_id)
    .bind(action)
    .bind(details)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_audit_logs(
    pool: &PgPool,
    guild_id: i64,
    limit: i64,
) -> AppResult<Vec<AuditLog>> {
    let results = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_logs WHERE guild_id = $1 ORDER BY created_at DESC LIMIT $2",
    )
    .bind(guild_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(results)
}
