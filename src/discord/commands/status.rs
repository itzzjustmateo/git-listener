use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let guild_count = ctx.cache().guild_count();
    let shard_info = ctx.serenity_context().shard_id;

    let latency = ctx.ping().await;
    ctx.say(format!(
        "Git Listener is running!\n\
         Servers: {guild_count}\n\
         Shard: {shard_info}\n\
         Latency: {}ms",
        latency.as_millis(),
    ))
    .await?;

    Ok(())
}
