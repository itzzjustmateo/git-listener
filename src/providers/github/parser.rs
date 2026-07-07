use super::types::GitHubPushPayload;
use crate::providers::types::*;
use crate::providers::ProviderError;

pub async fn parse_event(body: &[u8], event_type: &str) -> Result<RenderData, ProviderError> {
    match event_type {
        "push" => parse_push(body).await,
        "pull_request" => parse_pull_request(body).await,
        "issues" => parse_issue(body).await,
        "issue_comment" => parse_issue_comment(body).await,
        "release" => parse_release(body).await,
        "create" => parse_create(body).await,
        "delete" => parse_delete(body).await,
        "fork" => parse_fork(body).await,
        "watch" | "star" => parse_star(body).await,
        "gollum" => parse_wiki(body).await,
        "workflow_run" => parse_workflow_run(body).await,
        "deployment" => parse_deployment(body).await,
        "deployment_status" => parse_deployment_status(body).await,
        "discussion" => parse_discussion(body).await,
        "package" => parse_package(body).await,
        "repository" => parse_repository(body).await,
        _ => Err(ProviderError::UnsupportedEvent(event_type.to_owned())),
    }
}

async fn parse_push(body: &[u8]) -> Result<RenderData, ProviderError> {
    let payload: GitHubPushPayload = serde_json::from_slice(body)
        .map_err(|e| ProviderError::ParseError(e.to_string()))?;

    let branch = payload.r#ref.trim_start_matches("refs/heads/").to_owned();
    let commit_count = payload.commits.len();

    let mut data = RenderData::new("push");
    data.repository = Some(RepositoryInfo {
        id: payload.repository.id.map(|id| id.to_string()),
        name: payload.repository.name,
        full_name: payload.repository.full_name,
        owner: payload.repository.owner.login,
        owner_avatar_url: payload.repository.owner.avatar_url,
        url: Some(payload.repository.html_url),
        description: payload.repository.description,
        default_branch: Some(payload.repository.default_branch),
        private: payload.repository.private,
    });
    data.sender = Some(ActorInfo {
        id: payload.sender.id.map(|id| id.to_string()),
        login: payload.sender.login,
        display_name: None,
        avatar_url: payload.sender.avatar_url,
        url: Some(payload.sender.html_url),
    });
    data.ref_name = Some(payload.r#ref);
    data.branch = Some(branch);
    data.compare_url = Some(payload.compare);
    data.commit_count = commit_count;
    data.commits = payload
        .commits
        .into_iter()
        .map(|c| CommitInfo {
            id: c.id,
            message: c.message,
            author: ActorInfo {
                id: c.author.username,
                login: c.author.name,
                display_name: None,
                avatar_url: None,
                url: None,
            },
            url: Some(c.url),
            timestamp: None,
            added: c.added,
            removed: c.removed,
            modified: c.modified,
        })
        .collect();
    data.timestamp =
        Some(chrono::DateTime::from_timestamp(payload.repository.pushed_at as i64, 0).unwrap_or_default());

    Ok(data)
}

async fn parse_pull_request(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("pull_request".into()))
}

async fn parse_issue(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("issues".into()))
}

async fn parse_issue_comment(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("issue_comment".into()))
}

async fn parse_release(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("release".into()))
}

async fn parse_create(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("create".into()))
}

async fn parse_delete(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("delete".into()))
}

async fn parse_fork(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("fork".into()))
}

async fn parse_star(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("star".into()))
}

async fn parse_wiki(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("wiki".into()))
}

async fn parse_workflow_run(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("workflow_run".into()))
}

async fn parse_deployment(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("deployment".into()))
}

async fn parse_deployment_status(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("deployment_status".into()))
}

async fn parse_discussion(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("discussion".into()))
}

async fn parse_package(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("package".into()))
}

async fn parse_repository(_body: &[u8]) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent("repository".into()))
}
