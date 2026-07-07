use axum::http::HeaderMap;
use bytes::Bytes;

use crate::webhooks::WebhookError;

pub struct EventPipeline;

impl EventPipeline {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(
        &self,
        _provider_name: &str,
        _token: &str,
        _headers: HeaderMap,
        _body: Bytes,
    ) -> Result<(), WebhookError> {
        // TODO: Full pipeline implementation in Phase 7
        Ok(())
    }
}

impl Default for EventPipeline {
    fn default() -> Self {
        Self::new()
    }
}
