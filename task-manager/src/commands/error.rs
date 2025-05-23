#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Task not found")]
    NotFound,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}
