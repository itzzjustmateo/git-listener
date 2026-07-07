use sqlx::PgPool;

use crate::database::models::guild::Guild;
use crate::errors::AppResult;

pub async fn upsert_guild(pool: &PgPool, guild: &Guild) -> AppResult<Guild> {
    let result = sqlx::query_as::<_, Guild>(
        r#"
        INSERT INTO guilds (id, name, owner_id, preferred_locale, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name,
            owner_id = EXCLUDED.owner_id,
            preferred_locale = EXCLUDED.preferred_locale,
            updated_at = EXCLUDED.updated_at
        RETURNING *
        "#,
    )
    .bind(guild.id)
    .bind(&guild.name)
    .bind(guild.owner_id)
    .bind(&guild.preferred_locale)
    .bind(guild.created_at)
    .bind(guild.updated_at)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_guild(pool: &PgPool, guild_id: i64) -> AppResult<Option<Guild>> {
    let result = sqlx::query_as::<_, Guild>(
        "SELECT * FROM guilds WHERE id = $1",
    )
    .bind(guild_id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
