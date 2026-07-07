pub mod events;
pub mod parser;
pub mod signature;
pub mod types;

use async_trait::async_trait;
use axum::http::HeaderMap;

use super::traits::GitProvider;
use super::types::RenderData;
use super::{ProviderError, ProviderKind};

pub struct GitHubProvider;

#[async_trait]
impl GitProvider for GitHubProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::GitHub
    }

    fn verify_signature(
        &self,
        body: &[u8],
        signature_header: &str,
        secret: &[u8],
    ) -> Result<bool, ProviderError> {
        match crate::webhooks::signature::verify(
            ProviderKind::GitHub,
            body,
            signature_header,
            secret,
        ) {
            crate::webhooks::signature::VerificationResult::Valid => Ok(true),
            crate::webhooks::signature::VerificationResult::Invalid => Ok(false),
            crate::webhooks::signature::VerificationResult::NotSupported => {
                Err(ProviderError::SignatureVerificationFailed)
            }
        }
    }

    fn event_type_from_headers(
        &self,
        headers: &HeaderMap,
    ) -> Result<String, ProviderError> {
        headers
            .get("x-github-event")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned())
            .ok_or_else(|| {
                ProviderError::MissingHeader("x-github-event".into())
            })
    }

    async fn parse_event(
        &self,
        body: &[u8],
        event_type: &str,
    ) -> Result<RenderData, ProviderError> {
        parser::parse_event(body, event_type).await
    }
}
