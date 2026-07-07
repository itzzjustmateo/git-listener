use poise::serenity_prelude as serenity;

use crate::discord::client::{Context, Error};
use crate::discord::components::embeds;

#[poise::command(
    slash_command,
    subcommands("add", "remove", "list", "info"),
)]
pub async fn repository(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a subcommand: `add`, `remove`, `list`, or `info`").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Git provider (github, gitlab, gitea, bitbucket, azure-devops)"] provider: String,
    #[description = "Repository owner (user or organization)"] owner: String,
    #[description = "Repository name"] name: String,
    #[description = "Discord channel for notifications"] channel: serenity::Channel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let _guild_id = ctx.guild_id().ok_or_else(|| anyhow::anyhow!("must be used in a guild"))?;
    let full_name = format!("{owner}/{name}");

    // Validate provider
    provider.parse::<crate::providers::ProviderKind>()
        .map_err(|_| anyhow::anyhow!("Unknown provider: {provider}. Valid: github, gitlab, gitea, bitbucket, azure-devops"))?;

    ctx.say(format!("Repository `{full_name}` ({provider}) will be connected to <#{channel}>. Use `/provider configure` to set up the webhook secret."))
        .await?;

    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(format!("Repository `{repository}` disconnected.")).await?;
    Ok(())
}

#[poise::command(
    slash_command,
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let embed = embeds::info_embed(
        "Connected Repositories",
        "No repositories are currently connected. Use `/repository add` to connect one.",
    );

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}

#[poise::command(
    slash_command,
)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let embed = embeds::info_embed(
        format!("Repository: {repository}"),
        "Configuration details will be shown here.",
    );

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
