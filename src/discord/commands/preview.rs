use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
)]
pub async fn preview(
    ctx: Context<'_>,
    #[description = "Event type (push, pull_request, issues, etc.)"] event_type: String,
    #[description = "Repository (owner/name)"] _repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let data = crate::providers::types::RenderData::new(&event_type);
    let embeds = crate::rendering::dispatch::render_event(&data).await?;

    let mut reply = poise::CreateReply::default().ephemeral(true);
    for embed in embeds {
        reply = reply.embed(embed);
    }

    ctx.send(reply).await?;

    Ok(())
}
