use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;

pub async fn auth(req: Request, next: Next) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let state = req
        .extensions()
        .get::<crate::AppState>()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "internal state missing"})),
            )
        })?;

    let api_keys = &state.config.api.api_keys;

    if api_keys.is_empty() {
        return Ok(next.run(req).await);
    }

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "missing Authorization header"})),
            )
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "invalid Authorization format, expected Bearer <token>"})),
        )
    })?;

    if !api_keys.iter().any(|k| k == token) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "invalid API key"})),
        ));
    }

    Ok(next.run(req).await)
}
