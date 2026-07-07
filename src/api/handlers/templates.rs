use axum::Json;
use serde_json::Value;

pub async fn list_templates() -> Json<Value> {
    Json(serde_json::json!({"templates": []}))
}

pub async fn update_template() -> Json<Value> {
    Json(serde_json::json!({}))
}
