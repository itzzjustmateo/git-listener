use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub id: Option<String>,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub owner_avatar_url: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInfo {
    pub id: Option<String>,
    pub login: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: ActorInfo,
    pub url: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelInfo {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneInfo {
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestInfo {
    pub id: Option<String>,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>,
    pub state: String,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub additions: Option<i32>,
    pub deletions: Option<i32>,
    pub changed_files: Option<i32>,
    pub source_branch: String,
    pub target_branch: String,
    pub author: ActorInfo,
    pub assignees: Vec<ActorInfo>,
    pub reviewers: Vec<ActorInfo>,
    pub labels: Vec<LabelInfo>,
    pub milestone: Option<MilestoneInfo>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueInfo {
    pub id: Option<String>,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>,
    pub state: String,
    pub author: ActorInfo,
    pub assignees: Vec<ActorInfo>,
    pub labels: Vec<LabelInfo>,
    pub milestone: Option<MilestoneInfo>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub id: Option<String>,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub prerelease: bool,
    pub draft: bool,
    pub author: ActorInfo,
    pub created_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub id: Option<String>,
    pub name: String,
    pub run_number: Option<i64>,
    pub status: String,
    pub conclusion: Option<String>,
    pub url: Option<String>,
    pub branch: Option<String>,
    pub actor: Option<ActorInfo>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub id: Option<String>,
    pub environment: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionInfo {
    pub id: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>,
    pub category: Option<String>,
    pub author: ActorInfo,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub package_type: Option<String>,
    pub url: Option<String>,
    pub author: Option<ActorInfo>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiInfo {
    pub page: String,
    pub action: String,
    pub url: Option<String>,
    pub author: Option<ActorInfo>,
    pub diff_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentInfo {
    pub id: Option<String>,
    pub body: String,
    pub url: Option<String>,
    pub author: ActorInfo,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderData {
    pub repository: Option<RepositoryInfo>,
    pub sender: Option<ActorInfo>,
    pub commits: Vec<CommitInfo>,
    pub pull_request: Option<PullRequestInfo>,
    pub issue: Option<IssueInfo>,
    pub release: Option<ReleaseInfo>,
    pub workflow: Option<WorkflowInfo>,
    pub deployment: Option<DeploymentInfo>,
    pub discussion: Option<DiscussionInfo>,
    pub package: Option<PackageInfo>,
    pub comment: Option<CommentInfo>,
    pub wiki: Option<WikiInfo>,
    pub action: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub event_type: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub ref_name: Option<String>,
    pub ref_type: Option<String>,
    pub compare_url: Option<String>,
    pub commit_count: usize,
    pub additions: i32,
    pub deletions: i32,
    pub changed_files: i32,
}

impl RenderData {
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            repository: None,
            sender: None,
            commits: vec![],
            pull_request: None,
            issue: None,
            release: None,
            workflow: None,
            deployment: None,
            discussion: None,
            package: None,
            comment: None,
            wiki: None,
            action: "".into(),
            timestamp: None,
            event_type: event_type.into(),
            branch: None,
            tag: None,
            ref_name: None,
            ref_type: None,
            compare_url: None,
            commit_count: 0,
            additions: 0,
            deletions: 0,
            changed_files: 0,
        }
    }
}
