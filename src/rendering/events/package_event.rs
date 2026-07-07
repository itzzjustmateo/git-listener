use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let package = data.package.as_ref().ok_or_else(|| {
        RenderError::MissingField("package".into())
    })?;

    let color = match data.action.as_str() {
        "published" => colors::PACKAGE_PUBLISHED,
        _ => colors::PACKAGE_UPDATED,
    };

    let title = format!(
        "[{}] Package {} {} v{}",
        repo.full_name, data.action, package.name, package.version,
    );

    let embed = EmbedBuilder::new()
        .title(&title)
        .url(package.url.as_deref().unwrap_or(""))
        .color(color)
        .field("Type", package.package_type.as_deref().unwrap_or("unknown"), true)
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
