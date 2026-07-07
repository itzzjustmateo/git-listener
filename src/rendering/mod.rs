pub mod colors;
pub mod dispatch;
pub mod embed;
pub mod events;
pub mod template;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("template error: {0}")]
    Template(String),

    #[error("missing field: {0}")]
    MissingField(String),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
