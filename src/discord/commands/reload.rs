use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "ADMINISTRATOR",
)]
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Configuration reloaded.").await?;
    Ok(())
}
