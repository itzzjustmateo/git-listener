use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let title = format!("[{}] Forked by {}", repo.full_name, sender.login);

    let embed = EmbedBuilder::new()
        .title(&title)
        .color(colors::FORK)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
