use actix_web::web::Json;
use validator::{Validate, ValidationErrors};

use crate::driving::rest_handler::errors::ApiError;

pub fn validate<T>(params: &Json<T>) -> Result<(), ApiError>
    where
        T: Validate,
{
    match params.validate() {
        Ok(()) => Ok(()),
        Err(error) => Err(ApiError::ValidationError(collect_errors(error))),
    }
}

fn collect_errors(error: ValidationErrors) -> Vec<String> {
    error
        .field_errors()
        .into_iter()
        .map(|error| {
            let default_error = format!("{} is required", error.0);
            error.1[0]
                .message
                .as_ref()
                .unwrap_or(&std::borrow::Cow::Owned(default_error))
                .to_string()
        })
        .collect()
}
