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

    let color = match data.action.as_str() {
        "created" => colors::REPOSITORY_CREATED,
        "renamed" => colors::REPOSITORY_RENAMED,
        "archived" => colors::REPOSITORY_ARCHIVED,
        "transferred" => colors::REPOSITORY_TRANSFERRED,
        "privatized" | "publicized" => colors::REPOSITORY_VISIBILITY_CHANGED,
        _ => colors::DEFAULT,
    };

    let title = format!(
        "[{}] Repository {}",
        repo.full_name,
        data.action,
    );

    let embed = EmbedBuilder::new()
        .title(&title)
        .color(color)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
