use axum::Json;
use serde_json::Value;

pub async fn list_guilds() -> Json<Value> {
    Json(serde_json::json!({"guilds": []}))
}

pub async fn get_guild() -> Json<Value> {
    Json(serde_json::json!({}))
}

pub async fn update_guild() -> Json<Value> {
    Json(serde_json::json!({}))
}
