pub mod events;
pub mod parser;

use async_trait::async_trait;
use axum::http::HeaderMap;

use super::traits::GitProvider;
use super::types::RenderData;
use super::{ProviderError, ProviderKind};

pub struct AzureDevOpsProvider;

#[async_trait]
impl GitProvider for AzureDevOpsProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::AzureDevOps
    }

    fn verify_signature(
        &self,
        _body: &[u8],
        _signature_header: &str,
        _secret: &[u8],
    ) -> Result<bool, ProviderError> {
        // Azure DevOps uses Basic Auth or IP allowlisting
        Ok(true)
    }

    fn event_type_from_headers(
        &self,
        headers: &HeaderMap,
    ) -> Result<String, ProviderError> {
        headers
            .get("x-azuredevops-event-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned())
            .ok_or_else(|| {
                ProviderError::MissingHeader("x-azuredevops-event-type".into())
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
