use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("configure", "info"),
)]
pub async fn provider(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a subcommand: `configure` or `info`").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn configure(
    ctx: Context<'_>,
    #[description = "Provider name"] _provider: String,
    #[description = "Webhook secret"] _secret: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Provider secret updated.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
)]
pub async fn info(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let providers: Vec<&str> = crate::providers::ProviderKind::all()
        .iter()
        .map(|p| p.as_str())
        .collect();

    ctx.say(format!(
        "Supported providers: {}",
        providers.join(", "),
    ))
    .await?;

    Ok(())
}
