use crate::discord::client::{Context, Error};
use crate::discord::components::embeds;

#[poise::command(
    slash_command,
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let help_text = concat!(
        "**Git Listener** — Multi-provider Git webhook to Discord bot\n\n",
        "**Commands:**\n",
        "- `/setup` — Initial server setup\n",
        "- `/repository add` — Connect a repository\n",
        "- `/repository list` — List connected repositories\n",
        "- `/events enable/disable` — Control event types\n",
        "- `/filters add/remove` — Filter events by branch, user, etc.\n",
        "- `/templates set/reset` — Customize message templates\n",
        "- `/channel` — Set notification channel\n",
        "- `/provider configure` — Set webhook secret\n",
        "- `/test` — Send a test event\n",
        "- `/preview` — Preview event embeds\n",
        "- `/status` — Bot status\n",
        "- `/stats` — Event statistics\n",
        "- `/ping` — Latency check\n\n",
        "**Supported providers:** GitHub, GitLab, Gitea, Forgejo, Bitbucket, Azure DevOps\n",
        "**Webhook URL format:** `POST /webhook/{provider}/{token}`"
    );

    let embed = embeds::info_embed("Git Listener Help", help_text);

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
