use crate::discord::client::{Context, Error};
use crate::discord::components::embeds;

#[poise::command(
    slash_command,
)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let version = env!("CARGO_PKG_VERSION");

    let embed = embeds::info_embed(
        "Git Listener",
        format!(
            "A self-hosted Discord bot for Git webhook notifications.\n\n\
             Supports GitHub, GitLab, Gitea, Forgejo, Bitbucket, and Azure DevOps.\n\n\
             Version: {version}\n\
             Built with: Rust, Poise, Axum, SQLx, PostgreSQL"
        ),
    );

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
