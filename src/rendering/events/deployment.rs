use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let deployment = data.deployment.as_ref().ok_or_else(|| {
        RenderError::MissingField("deployment".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let title = format!(
        "[{}] Deployment to {} — {}",
        repo.full_name,
        deployment.environment,
        deployment.status.as_deref().unwrap_or("created"),
    );

    let embed = EmbedBuilder::new()
        .title(&title)
        .url(deployment.url.as_deref().unwrap_or(""))
        .color(colors::DEPLOYMENT)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .field("Environment", &deployment.environment, true)
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
