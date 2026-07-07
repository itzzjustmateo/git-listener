use poise::serenity_prelude as serenity;

use crate::providers::types::RenderData;

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub async fn dispatch(
        &self,
        _channel_id: u64,
        _embeds: Vec<serenity::CreateEmbed>,
        _data: &RenderData,
    ) -> Result<(), crate::errors::AppError> {
        // TODO: Full dispatch implementation in Phase 8
        Ok(())
    }
}
