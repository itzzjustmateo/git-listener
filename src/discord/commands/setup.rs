
use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "ADMINISTRATOR",
)]
pub async fn setup(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_id = ctx.guild_id().ok_or_else(|| anyhow::anyhow!("this command must be used in a guild"))?;
    let guild = guild_id
        .to_partial_guild(&ctx.serenity_context().http)
        .await?;

    let guild_model = crate::database::models::guild::Guild {
        id: guild_id.get() as i64,
        name: guild.name.clone(),
        owner_id: guild.owner_id.get() as i64,
        preferred_locale: "en-US".into(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    crate::database::queries::guild::upsert_guild(&ctx.data().state.db, &guild_model).await?;

    ctx.say("Git Listener has been set up for this server! Use `/repository add` to connect a repository.")
        .await?;

    Ok(())
}
