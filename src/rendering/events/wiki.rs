use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let wiki = data.wiki.as_ref().ok_or_else(|| {
        RenderError::MissingField("wiki".into())
    })?;

    let title = format!(
        "[{}] Wiki page {} — {}",
        repo.full_name, wiki.action, wiki.page,
    );

    let embed = EmbedBuilder::new()
        .title(&title)
        .url(wiki.url.as_deref().unwrap_or(""))
        .color(colors::WIKI)
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
