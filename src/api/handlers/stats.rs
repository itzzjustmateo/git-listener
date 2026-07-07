use axum::Json;
use serde_json::Value;

pub async fn get_statistics() -> Json<Value> {
    Json(serde_json::json!({"statistics": {}}))
}
