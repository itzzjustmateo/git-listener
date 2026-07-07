use axum::Json;
use serde_json::Value;

pub async fn list_filters() -> Json<Value> {
    Json(serde_json::json!({"filters": []}))
}

pub async fn create_filter() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn delete_filter() -> Json<Value> {
    Json(serde_json::json!({}))
}
