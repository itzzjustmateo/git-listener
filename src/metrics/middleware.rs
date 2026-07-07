use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

use crate::AppState;

pub async fn track_webhook_metrics(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let path = request.uri().path().to_owned();

    let response = next.run(request).await;

    let duration = start.elapsed();
    tracing::debug!("{path} took {duration:?}");

    response
}
