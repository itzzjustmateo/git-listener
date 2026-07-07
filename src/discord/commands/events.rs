use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("enable", "disable", "list"),
)]
pub async fn events(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a subcommand: `enable`, `disable`, or `list`").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn enable(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Event type (push, pull_request, issues, etc.)"] _event_type: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Event enabled.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn disable(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Event type to disable"] _event_type: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Event disabled.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Event configuration will be shown here.").await?;
    Ok(())
}
