use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("set", "reset", "show"),
)]
pub async fn templates(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a subcommand: `set`, `reset`, or `show`").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Event type (push, pull_request, etc.)"] _event_type: String,
    #[description = "Template content with {{placeholders}}"] _template: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Template updated.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn reset(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Event type"] _event_type: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Template reset to default.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
)]
pub async fn show(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Templates will be shown here.").await?;
    Ok(())
}
