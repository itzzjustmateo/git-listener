use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("invalid signature")]
    InvalidSignature,

    #[error("missing signature header")]
    MissingSignature,

    #[error("rate limited")]
    RateLimited,

    #[error("unknown provider: {0}")]
    UnknownProvider(String),

    #[error("invalid endpoint token")]
    InvalidToken,

    #[error("repository not found for token")]
    RepositoryNotFound,

    #[error("unknown event type: {0}")]
    UnknownEventType(String),

    #[error("payload too large")]
    PayloadTooLarge,

    #[error("malformed payload: {0}")]
    MalformedPayload(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl WebhookError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
            Self::MissingSignature => StatusCode::UNAUTHORIZED,
            Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Self::UnknownProvider(_) => StatusCode::NOT_FOUND,
            Self::InvalidToken => StatusCode::NOT_FOUND,
            Self::RepositoryNotFound => StatusCode::NOT_FOUND,
            Self::UnknownEventType(_) => StatusCode::BAD_REQUEST,
            Self::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::MalformedPayload(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for WebhookError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": self.to_string(),
            "code": status.as_u16(),
        }));
        (status, body).into_response()
    }
}
