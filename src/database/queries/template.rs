use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::template::Template;
use crate::errors::AppResult;

pub async fn get_template(
    pool: &PgPool,
    repository_id: Uuid,
    event_type: &str,
) -> AppResult<Option<Template>> {
    let result = sqlx::query_as::<_, Template>(
        "SELECT * FROM templates WHERE repository_id = $1 AND event_type = $2 AND is_active = true",
    )
    .bind(repository_id)
    .bind(event_type)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn upsert_template(
    pool: &PgPool,
    repository_id: Uuid,
    event_type: &str,
    template_content: &str,
) -> AppResult<Template> {
    let result = sqlx::query_as::<_, Template>(
        r#"
        INSERT INTO templates (repository_id, event_type, template_content)
        VALUES ($1, $2, $3)
        ON CONFLICT (repository_id, event_type) DO UPDATE SET
            template_content = EXCLUDED.template_content,
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(repository_id)
    .bind(event_type)
    .bind(template_content)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
