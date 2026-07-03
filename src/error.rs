use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompletedError {
    #[error("error: {0}")]
    NotificationError(String),

    #[error("unknown profile '{0}', check the configuration file")]
    UnknownProfile(String),

    #[error("unknown destination '{0}', check the configuration file")]
    UnknownDestination(String),
}
