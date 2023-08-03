use std::fmt;
use std::fmt::Display;

use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(Vec<String>),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BadRequest(err)
                | ApiError::InternalServerError(err)
                | ApiError::NotFound(err)
                | ApiError::InvalidData(err)
                | ApiError::Conflict(err)
                | ApiError::Unknown(err) => writeln!(f, "{},", err),
            ApiError::ValidationError(mex_vec) => {
                mex_vec.iter().fold(Ok(()), |result, err| {
                    result.and_then(|_| writeln!(f, "{}, ", err))
                })
            },
        }
    }
}

/// Automatically convert ApiErrors to external ResponseError
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json(error)
            }
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json(message)
            }
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(&errors.to_vec())
            }
            ApiError::InternalServerError(error) => {
                HttpResponse::Unauthorized().json(error)
            }
            ApiError::Conflict(error) => {
                HttpResponse::Conflict().json(error)
            }
            ApiError::InvalidData(error) => {
                HttpResponse::BadRequest().json(error)
            }
            ApiError::Unknown(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
