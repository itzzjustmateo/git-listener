use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn test(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(format!("Test event sent for `{repository}`. Check your configured channel.")).await?;
    Ok(())
}
