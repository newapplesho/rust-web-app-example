use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use service::error::service_error::ServiceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiErrorResponse {
    #[error("Requested resource not found")]
    NotFound,

    #[error("Internal server error has occurred")]
    InternalServerError,

    #[error("{0}")]
    BadRequest(String),
}

impl ApiErrorResponse {
    pub fn error_type(&self) -> String {
        match self {
            Self::NotFound => "not_found".to_string(),
            Self::InternalServerError => "internal_server_error".to_string(),
            Self::BadRequest(_) => "bad_request".to_string(),
        }
    }
}

impl ResponseError for ApiErrorResponse {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error_type: self.error_type(),
            error_message: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorResponse::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorResponse::NotFound => StatusCode::NOT_FOUND,
            ApiErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error_type: String,
    error_message: String,
}

pub fn map_service_error(e: ServiceError) -> ApiErrorResponse {
    match e {
        ServiceError::ResourceNotFound => ApiErrorResponse::NotFound,
        ServiceError::BadRequest(message) => ApiErrorResponse::BadRequest(message),
        _ => ApiErrorResponse::InternalServerError,
    }
}
