use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "ADMINISTRATOR",
)]
pub async fn permissions(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Permission management will be shown here.").await?;
    Ok(())
}
