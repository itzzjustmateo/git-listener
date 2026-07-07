use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let issue = data.issue.as_ref().ok_or_else(|| {
        RenderError::MissingField("issue".into())
    })?;
    let comment = data.comment.as_ref().ok_or_else(|| {
        RenderError::MissingField("comment".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let title = format!(
        "[{}] Comment on Issue #{} — {}",
        repo.full_name, issue.number, issue.title,
    );

    let truncated = if comment.body.len() > 300 {
        format!("{}…", &comment.body[..300])
    } else {
        comment.body.clone()
    };

    let embed = EmbedBuilder::new()
        .title(&title)
        .url(comment.url.as_deref().unwrap_or(""))
        .color(colors::ISSUE_COMMENT)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .description(truncated)
        .timestamp(comment.updated_at.unwrap_or_else(chrono::Utc::now))
        .build();

    Ok(vec![embed])
}
