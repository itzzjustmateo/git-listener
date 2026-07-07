use axum::extract::{Path, Request, State};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;

use crate::AppState;

use super::error::WebhookError;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/webhook/{provider}/{token}", post(handle_webhook))
        .route("/health", axum::routing::get(health))
        .layer(tower::ServiceBuilder::new().layer(axum::middleware::from_fn_with_state(
            state.clone(),
            super::middleware::body_limit,
        )))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn handle_webhook(
    State(_state): State<AppState>,
    Path((provider, token)): Path<(String, String)>,
    _request: Request,
) -> Result<Json<Value>, WebhookError> {
    tracing::debug!("webhook received: provider={provider}, token={token}");

    // TODO: Phase 7 — full pipeline integration
    tracing::warn!("webhook handler not yet implemented");

    Ok(Json(serde_json::json!({"status": "not_implemented"})))
}
