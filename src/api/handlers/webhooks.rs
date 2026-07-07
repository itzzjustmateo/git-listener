use axum::Json;
use serde_json::Value;

pub async fn get_webhook_endpoint() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn rotate_webhook_secret() -> Json<Value> {
    Json(serde_json::json!({}))
}
