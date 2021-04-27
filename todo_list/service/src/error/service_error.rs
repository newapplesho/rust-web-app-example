use domain::error::domain_error::DomainError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ServiceError {
    #[error("Requested resource not found")]
    ResourceNotFound,

    #[error("Unexpected domain error")]
    UnexpectedServiceError,

    #[error("{0}")]
    BadRequest(String),
}

pub fn map_domain_error(e: DomainError) -> ServiceError {
    match e {
        DomainError::ResourceNotFound => ServiceError::ResourceNotFound,
        DomainError::BadRequest(message) => ServiceError::BadRequest(message),
        _ => ServiceError::UnexpectedServiceError,
    }
}
