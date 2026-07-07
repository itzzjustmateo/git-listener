use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;

use super::RenderError;

pub async fn render_event(
    data: &RenderData,
) -> Result<Vec<serenity::CreateEmbed>, RenderError> {
    match data.event_type.as_str() {
        "push" => super::events::push::render(data),
        "pull_request" => super::events::pull_request::render(data),
        "issues" => super::events::issue::render(data),
        "issue_comment" => super::events::issue_comment::render(data),
        "release" => super::events::release::render(data),
        "create" | "delete" => super::events::branch::render(data),
        "fork" => super::events::fork::render(data),
        "watch" | "star" => super::events::star::render(data),
        "gollum" => super::events::wiki::render(data),
        "workflow_run" => super::events::workflow::render(data),
        "deployment" | "deployment_status" => super::events::deployment::render(data),
        "discussion" => super::events::discussion::render(data),
        "package" => super::events::package_event::render(data),
        "repository" => super::events::repository::render(data),
        _ => super::events::generic::render(data),
    }
}
