use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;

    let title = format!(
        "[{}] Event: {} — {}",
        repo.full_name,
        data.event_type,
        data.action,
    );

    let embed = EmbedBuilder::new()
        .title(&title)
        .color(colors::DEFAULT)
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
