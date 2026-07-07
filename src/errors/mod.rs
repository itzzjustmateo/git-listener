use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("webhook error: {0}")]
    Webhook(#[from] crate::webhooks::WebhookError),

    #[error("provider error: {0}")]
    Provider(#[from] crate::providers::ProviderError),

    #[error("rendering error: {0}")]
    Rendering(#[from] crate::rendering::RenderError),

    #[error("discord error: {0}")]
    Discord(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Validation(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Validation(s.to_owned())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Webhook(e) => (e.status_code(), self.to_string()),
            Self::NotFound(_) => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            Self::Validation(_) => (axum::http::StatusCode::BAD_REQUEST, self.to_string()),
            Self::Config(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "configuration error".into(),
            ),
            Self::Database(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "database error".into(),
            ),
            Self::Provider(e) => (e.status_code(), self.to_string()),
            Self::Rendering(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "rendering error".into(),
            ),
            Self::Discord(_) => (
                axum::http::StatusCode::BAD_GATEWAY,
                "discord API error".into(),
            ),
            Self::Internal(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal error".into(),
            ),
        };

        let body = Json(json!({
            "error": message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
