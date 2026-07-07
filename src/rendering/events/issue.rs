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
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let color = match data.action.as_str() {
        "opened" | "reopened" => colors::ISSUES_OPENED,
        "closed" => colors::ISSUES_CLOSED,
        _ => colors::DEFAULT,
    };

    let title = format!(
        "[{}] Issue #{} {} — {}",
        repo.full_name, issue.number, data.action, issue.title,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(issue.url.as_deref().unwrap_or(""))
        .color(color)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(issue.updated_at.unwrap_or_else(chrono::Utc::now));

    if let Some(body) = &issue.body {
        let truncated = if body.len() > 300 {
            format!("{}…", &body[..300])
        } else {
            body.clone()
        };
        embed = embed.description(truncated);
    }

    if !issue.labels.is_empty() {
        let labels: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();
        embed = embed.field("Labels", labels.join(", "), true);
    }

    if let Some(milestone) = &issue.milestone {
        embed = embed.field("Milestone", &milestone.title, true);
    }

    Ok(vec![embed.build()])
}
