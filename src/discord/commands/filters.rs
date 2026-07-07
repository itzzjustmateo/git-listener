use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("add", "remove", "list"),
)]
pub async fn filters(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a subcommand: `add`, `remove`, or `list`").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Filter type: branch, tag, user, file, label, milestone, event_type"] _filter_type: String,
    #[description = "Value to match (supports glob patterns)"] _value: String,
    #[description = "Action: include or exclude"] _action: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Filter added.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
    #[description = "Filter ID to remove"] _filter_id: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Filter removed.").await?;
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
    ctx.say("Filters will be listed here.").await?;
    Ok(())
}
