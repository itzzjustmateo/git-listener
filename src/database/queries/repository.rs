use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::repository::Repository;
use crate::errors::AppResult;

pub async fn list_repositories(pool: &PgPool, guild_id: i64) -> AppResult<Vec<Repository>> {
    let results = sqlx::query_as::<_, Repository>(
        "SELECT * FROM repositories WHERE guild_id = $1 AND is_active = true ORDER BY full_name",
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn get_repository(pool: &PgPool, id: Uuid) -> AppResult<Option<Repository>> {
    let result = sqlx::query_as::<_, Repository>(
        "SELECT * FROM repositories WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn get_repository_by_full_name(
    pool: &PgPool,
    guild_id: i64,
    full_name: &str,
) -> AppResult<Option<Repository>> {
    let result = sqlx::query_as::<_, Repository>(
        "SELECT * FROM repositories WHERE guild_id = $1 AND full_name = $2",
    )
    .bind(guild_id)
    .bind(full_name)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

pub async fn create_repository(pool: &PgPool, repo: &Repository) -> AppResult<Repository> {
    let result = sqlx::query_as::<_, Repository>(
        r#"
        INSERT INTO repositories (guild_id, provider, provider_repo_id, owner_name, repo_name,
            full_name, webhook_secret_hash, destination_channel_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(repo.guild_id)
    .bind(&repo.provider)
    .bind(&repo.provider_repo_id)
    .bind(&repo.owner_name)
    .bind(&repo.repo_name)
    .bind(&repo.full_name)
    .bind(&repo.webhook_secret_hash)
    .bind(repo.destination_channel_id)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn delete_repository(pool: &PgPool, id: Uuid) -> AppResult<Option<Repository>> {
    let result = sqlx::query_as::<_, Repository>(
        "UPDATE repositories SET is_active = false, updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
