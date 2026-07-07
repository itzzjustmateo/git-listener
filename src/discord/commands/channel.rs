use poise::serenity_prelude as serenity;

use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn channel(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Destination channel"] _channel: serenity::Channel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Notification channel updated.").await?;
    Ok(())
}
