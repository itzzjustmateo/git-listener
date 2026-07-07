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
    let branch = data.branch.as_deref().unwrap_or("unknown");
    let commit_count = data.commit_count;

    let title = format!(
        "[{}] {} commit{} to {}",
        repo.full_name,
        commit_count,
        if commit_count == 1 { "" } else { "s" },
        branch,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(data.compare_url.as_deref().unwrap_or(""))
        .color(colors::PUSH)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .timestamp(data.timestamp.unwrap_or_else(chrono::Utc::now))
        .thumbnail(repo.owner_avatar_url.clone().unwrap_or_default());

    if commit_count > 0 {
        let commit_lines: Vec<String> = data
            .commits
            .iter()
            .take(5)
            .map(|c| {
                let msg = c.message.lines().next().unwrap_or("");
                format!("[`{}`]({}) {}", &c.id[..7], c.url.as_deref().unwrap_or(""), msg)
            })
            .collect();

        let mut desc = commit_lines.join("\n");

        if commit_count > 5 {
            desc.push_str(&format!(
                "\n… and {} more commit{}",
                commit_count - 5,
                if commit_count - 5 == 1 { "" } else { "s" },
            ));
        }

        embed = embed.description(desc);
    }

    if data.additions > 0 || data.deletions > 0 {
        embed = embed.field(
            "Changes",
            format!(
                "+{} -{}  {} files",
                data.additions, data.deletions, data.changed_files
            ),
            true,
        );
    }

    Ok(vec![embed.build()])
}
