use axum::Json;
use serde_json::Value;

pub async fn list_repositories() -> Json<Value> {
    Json(serde_json::json!({"repositories": []}))
}

pub async fn create_repository() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn get_repository() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn update_repository() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn delete_repository() -> Json<Value> {
    Json(serde_json::json!({}))
}
