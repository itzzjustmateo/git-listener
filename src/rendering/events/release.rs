use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;
use crate::rendering::colors;
use crate::rendering::embed::EmbedBuilder;
use crate::rendering::RenderError;

pub fn render(data: &RenderData) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    let repo = data.repository.as_ref().ok_or_else(|| {
        RenderError::MissingField("repository".into())
    })?;
    let release = data.release.as_ref().ok_or_else(|| {
        RenderError::MissingField("release".into())
    })?;
    let sender = data.sender.as_ref().ok_or_else(|| {
        RenderError::MissingField("sender".into())
    })?;

    let color = if release.prerelease {
        colors::RELEASE_PRERELEASE
    } else {
        colors::RELEASE
    };

    let release_name = release
        .name
        .as_deref()
        .unwrap_or(&release.tag_name);

    let title = format!(
        "[{}] {} {}",
        repo.full_name,
        data.action,
        release_name,
    );

    let mut embed = EmbedBuilder::new()
        .title(&title)
        .url(release.url.as_deref().unwrap_or(""))
        .color(color)
        .author(
            &sender.login,
            sender.avatar_url.clone(),
            sender.url.clone(),
        )
        .field("Tag", &release.tag_name, true)
        .timestamp(release.published_at.unwrap_or_else(chrono::Utc::now));

    if release.prerelease {
        embed = embed.field("Pre-release", "Yes", true);
    }
    if release.draft {
        embed = embed.field("Draft", "Yes", true);
    }

    if let Some(body) = &release.body {
        let truncated = if body.len() > 300 {
            format!("{}…", &body[..300])
        } else {
            body.clone()
        };
        embed = embed.description(truncated);
    }

    Ok(vec![embed.build()])
}
