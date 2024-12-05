
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Authentication failed")]
    AuthError,
    #[error("Item not found")]
    NotFound,
    #[error("Insufficient inventory")]
    InsufficientInventory,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}
