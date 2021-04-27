use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Requested resource not found")]
    ResourceNotFound,

    #[error("Unexpected error from Database")]
    UnexpectedDatabaseError,

    #[error("{0}")]
    BadRequest(String),
}
