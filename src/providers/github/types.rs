use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubPushPayload {
    pub r#ref: String,
    pub before: String,
    pub after: String,
    pub compare: String,
    pub commits: Vec<GitHubCommit>,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
    pub forced: Option<bool>,
    pub created: Option<bool>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: GitHubCommitAuthor,
    pub committer: GitHubCommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommitAuthor {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRepository {
    pub id: Option<i64>,
    pub name: String,
    pub full_name: String,
    pub owner: GitHubOwner,
    pub html_url: String,
    pub description: Option<String>,
    pub default_branch: String,
    pub private: bool,
    pub pushed_at: f64,
    pub created_at: f64,
    pub updated_at: f64,
}

#[derive(Debug, Deserialize)]
pub struct GitHubOwner {
    pub id: Option<i64>,
    pub login: String,
    pub avatar_url: Option<String>,
    pub html_url: String,
    #[serde(rename = "type")]
    pub owner_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubSender {
    pub id: Option<i64>,
    pub login: String,
    pub avatar_url: Option<String>,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPullRequestPayload {
    pub action: String,
    pub number: i64,
    pub pull_request: GitHubPullRequest,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPullRequest {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub state: String,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub additions: Option<i32>,
    pub deletions: Option<i32>,
    pub changed_files: Option<i32>,
    pub head: GitHubPullRequestBranch,
    pub base: GitHubPullRequestBranch,
    pub user: GitHubSender,
    pub assignees: Vec<GitHubSender>,
    pub labels: Vec<GitHubLabel>,
    pub milestone: Option<GitHubMilestone>,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub closed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPullRequestBranch {
    pub r#ref: String,
    pub sha: String,
    pub repo: Option<GitHubRepository>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubLabel {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubMilestone {
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub due_on: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubIssuePayload {
    pub action: String,
    pub issue: GitHubIssue,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubIssue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub state: String,
    pub user: GitHubSender,
    pub assignees: Vec<GitHubSender>,
    pub labels: Vec<GitHubLabel>,
    pub milestone: Option<GitHubMilestone>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCommentPayload {
    pub action: String,
    pub issue: GitHubIssue,
    pub comment: GitHubComment,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubComment {
    pub id: i64,
    pub body: String,
    pub html_url: String,
    pub user: GitHubSender,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubReleasePayload {
    pub action: String,
    pub release: GitHubRelease,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub id: i64,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    pub prerelease: bool,
    pub draft: bool,
    pub author: GitHubSender,
    pub created_at: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubCreateDeletePayload {
    pub r#ref: String,
    pub ref_type: String,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubForkPayload {
    pub forkee: GitHubRepository,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubStarPayload {
    pub action: String,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubWikiPayload {
    pub pages: Vec<GitHubWikiPage>,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubWikiPage {
    pub page_name: String,
    pub title: String,
    pub action: String,
    pub html_url: String,
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubWorkflowRunPayload {
    pub action: String,
    pub workflow_run: GitHubWorkflowRun,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubWorkflowRun {
    pub id: i64,
    pub name: String,
    pub run_number: i64,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub head_branch: Option<String>,
    pub actor: GitHubSender,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubDeploymentPayload {
    pub action: String,
    pub deployment: GitHubDeployment,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubDeployment {
    pub id: i64,
    pub environment: String,
    pub description: Option<String>,
    pub creator: GitHubSender,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubDeploymentStatusPayload {
    pub action: String,
    pub deployment_status: GitHubDeploymentStatus,
    pub deployment: GitHubDeployment,
    pub repository: GitHubRepository,
    pub sender: GitHubSender,
}

#[derive(Debug, Deserialize)]
pub struct GitHubDeploymentStatus {
    pub id: i64,
    pub state: String,
    pub description: Option<String>,
    pub environment: Option<String>,
    pub log_url: Option<String>,
    pub deployment_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
