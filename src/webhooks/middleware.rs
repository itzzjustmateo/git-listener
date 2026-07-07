use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use http_body_util::BodyExt;

use crate::AppState;

use super::error::WebhookError;

pub async fn body_limit(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, WebhookError> {
    let max_size = state.config.webhook.max_body_size;

    let (parts, body) = request.into_parts();
    let body_bytes = body
        .collect()
        .await
        .map_err(|_| WebhookError::MalformedPayload("failed to read body".into()))?
        .to_bytes();

    if body_bytes.len() > max_size {
        return Err(WebhookError::PayloadTooLarge);
    }

    let request = Request::from_parts(parts, Body::from(body_bytes));
    Ok(next.run(request).await)
}
