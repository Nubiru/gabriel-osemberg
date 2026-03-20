//! Domain-specific error types for the DATA layer.

#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum DataError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

// Note: DataError automatically converts to ServerFnError via the blanket
// impl<E: StdError> From<E> for ServerFnError in Leptos. No manual impl needed.
