use crate::providers::types::*;
use crate::providers::ProviderError;

pub async fn parse_event(_body: &[u8], event_type: &str) -> Result<RenderData, ProviderError> {
    Err(ProviderError::UnsupportedEvent(event_type.to_owned()))
}
