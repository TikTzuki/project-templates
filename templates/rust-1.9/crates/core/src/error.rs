//! Shared error types.

/// Application error type.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// An unexpected internal error.
    #[error("internal error: {0}")]
    Internal(String),

    /// Wraps an [`anyhow::Error`].
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
