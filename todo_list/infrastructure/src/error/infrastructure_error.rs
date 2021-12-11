use domain::error::domain_error::DomainError;
use log::error;

pub fn map_sqlx_error(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::ResourceNotFound,
        _ => {
            error!("Unexpected error occurred in sqlx: {:?}", e);
            DomainError::UnexpectedDatabaseError
        }
    }
}
