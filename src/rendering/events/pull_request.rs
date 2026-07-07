use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let pr = data.pull_request.as_ref().ok_or_else(|| {
        RenderError::MissingField("pull_request".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let color = match data.action.as_str() {
        "opened" | "reopened" => colors::PULL_REQUEST_OPENED,
        "closed" if pr.merged => colors::PULL_REQUEST_MERGED,
        "closed" => colors::PULL_REQUEST_CLOSED,
        _ => colors::PULL_REQUEST_OPENED,
    };

    let action_label = match data.action.as_str() {
        "opened" => "opened",
        "closed" if pr.merged => "merged",
        "closed" => "closed",
        "reopened" => "reopened",
        "synchronize" => "updated",
        "review_requested" => "review requested",
        "ready_for_review" => "marked ready",
        _ => &data.action,
    };

    let title = format!(
        "[{}] PR #{} {} — {}",
        repo.full_name, pr.number, action_label, pr.title,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(pr.url.as_deref().unwrap_or(""))
        .color(color)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(pr.updated_at.unwrap_or_else(chrono::Utc::now));

    if let Some(body) = &pr.body {
        let truncated = if body.len() > 200 {
            format!("{}…", &body[..200])
        } else {
            body.clone()
        };
        embed = embed.description(truncated);
    }

    embed = embed
        .field("Branch", format!("{} → {}", pr.source_branch, pr.target_branch), true)
        .field("Additions", pr.additions.unwrap_or(0).to_string(), true)
        .field("Deletions", pr.deletions.unwrap_or(0).to_string(), true);

    Ok(vec![embed.build()])
}
