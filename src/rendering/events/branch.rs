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

    let ref_type = data.ref_type.as_deref().unwrap_or("unknown");
    let ref_name = data.ref_name.as_deref().unwrap_or("unknown");
    let is_create = data.event_type == "create" || data.action == "created";

    let color = if is_create {
        if ref_type == "tag" {
            colors::TAG_CREATED
        } else {
            colors::BRANCH_CREATED
        }
    } else if ref_type == "tag" {
        colors::TAG_DELETED
    } else {
        colors::BRANCH_DELETED
    };

    let action = if is_create { "created" } else { "deleted" };

    let title = format!(
        "[{}] {} {}: {}",
        repo.full_name, ref_type, action, ref_name,
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
