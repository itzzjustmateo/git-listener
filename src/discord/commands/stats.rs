use crate::discord::client::{Context, Error};

#[poise::command(
    slash_command,
)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Repository (owner/name)"] _repository: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Statistics will be shown here.").await?;
    Ok(())
}
