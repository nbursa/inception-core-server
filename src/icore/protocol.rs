use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IcoreError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Model error: {0}")]
    Model(String),
}
