pub mod azure_devops;
pub mod bitbucket;
pub mod gitea_family;
pub mod github;
pub mod gitlab;
pub mod traits;
pub mod types;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    GitHub,
    GitLab,
    GiteaFamily,
    Bitbucket,
    AzureDevOps,
}

impl ProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GitHub => "github",
            Self::GitLab => "gitlab",
            Self::GiteaFamily => "gitea",
            Self::Bitbucket => "bitbucket",
            Self::AzureDevOps => "azure-devops",
        }
    }

    pub fn all() -> &'static [ProviderKind] {
        &[
            Self::GitHub,
            Self::GitLab,
            Self::GiteaFamily,
            Self::Bitbucket,
            Self::AzureDevOps,
        ]
    }
}

impl FromStr for ProviderKind {
    type Err = ProviderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Self::GitHub),
            "gitlab" => Ok(Self::GitLab),
            "gitea" | "forgejo" | "gitea_family" => Ok(Self::GiteaFamily),
            "bitbucket" => Ok(Self::Bitbucket),
            "azure-devops" | "azuredevops" | "azure_devops" => Ok(Self::AzureDevOps),
            _ => Err(ProviderError::UnknownProvider(s.to_owned())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("unknown provider: {0}")]
    UnknownProvider(String),

    #[error("unknown event type: {0}")]
    UnknownEventType(String),

    #[error("missing header: {0}")]
    MissingHeader(String),

    #[error("signature verification failed")]
    SignatureVerificationFailed,

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("unsupported event: {0}")]
    UnsupportedEvent(String),
}

impl ProviderError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::UnknownProvider(_) => axum::http::StatusCode::NOT_FOUND,
            Self::UnknownEventType(_) => axum::http::StatusCode::BAD_REQUEST,
            Self::MissingHeader(_) => axum::http::StatusCode::BAD_REQUEST,
            Self::SignatureVerificationFailed => axum::http::StatusCode::UNAUTHORIZED,
            Self::ParseError(_) => axum::http::StatusCode::BAD_REQUEST,
            Self::UnsupportedEvent(_) => axum::http::StatusCode::BAD_REQUEST,
        }
    }
}
