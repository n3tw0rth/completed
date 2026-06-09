use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompletedError {
    #[error("error: {0}")]
    NotificationError(String),
}
