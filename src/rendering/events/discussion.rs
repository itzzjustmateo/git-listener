use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let discussion = data.discussion.as_ref().ok_or_else(|| {
        RenderError::MissingField("discussion".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let title = format!(
        "[{}] Discussion {} — {}",
        repo.full_name, data.action, discussion.title,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(discussion.url.as_deref().unwrap_or(""))
        .color(colors::DISCUSSION)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(discussion.updated_at.unwrap_or_else(chrono::Utc::now));

    if let Some(body) = &discussion.body {
        let truncated = if body.len() > 300 {
            format!("{}…", &body[..300])
        } else {
            body.clone()
        };
        embed = embed.description(truncated);
    }

    if let Some(category) = &discussion.category {
        embed = embed.field("Category", category, true);
    }

    Ok(vec![embed.build()])
}
