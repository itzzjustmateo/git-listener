use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    ctx.say(format!("Pong! Latency: {}ms", latency.as_millis())).await?;
    Ok(())
}
