use async_trait::async_trait;
use axum::http::HeaderMap;

use super::types::RenderData;
use super::{ProviderError, ProviderKind};

#[async_trait]
pub trait GitProvider: Send + Sync {
    fn kind(&self) -> ProviderKind;

    fn verify_signature(
        &self,
        body: &[u8],
        signature_header: &str,
        secret: &[u8],
    ) -> Result<bool, ProviderError>;

    fn event_type_from_headers(
        &self,
        headers: &HeaderMap,
    ) -> Result<String, ProviderError>;

    async fn parse_event(
        &self,
        body: &[u8],
        event_type: &str,
    ) -> Result<RenderData, ProviderError>;
}
