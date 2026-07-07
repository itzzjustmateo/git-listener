use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let workflow = data.workflow.as_ref().ok_or_else(|| {
        RenderError::MissingField("workflow".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let color = match workflow.conclusion.as_deref() {
        Some("success") => colors::WORKFLOW_SUCCESS,
        Some("failure") | Some("cancelled") | Some("timed_out") => colors::WORKFLOW_FAILURE,
        Some("neutral") => colors::DEFAULT,
        _ => match workflow.status.as_str() {
            "in_progress" | "queued" => colors::WORKFLOW_IN_PROGRESS,
            "pending" => colors::WORKFLOW_PENDING,
            _ => colors::DEFAULT,
        },
    };

    let status = workflow
        .conclusion
        .as_deref()
        .unwrap_or(&workflow.status);

    let title = format!(
        "[{}] Workflow {} — {}",
        repo.full_name, workflow.name, status,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(workflow.url.as_deref().unwrap_or(""))
        .color(color)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now));

    if let Some(branch) = &workflow.branch {
        embed = embed.field("Branch", branch, true);
    }
    if let Some(run_number) = workflow.run_number {
        embed = embed.field("Run", format!("#{run_number}"), true);
    }
    if let Some(actor) = &workflow.actor {
        embed = embed.field("Actor", &actor.login, true);
    }

    Ok(vec![embed.build()])
}
