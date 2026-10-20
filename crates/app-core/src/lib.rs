#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
}
