pub mod client;
pub mod commands;
pub mod components;

use crate::AppState;

pub async fn start(state: AppState) -> anyhow::Result<()> {
    client::run(state).await
}
