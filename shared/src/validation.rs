//! HTTP request validation

use super::error::{ApiError, ApiResult};
use serde_json::json;
use validator::Validate;

/// Validate the struct data
pub fn validate_data<T: Validate>(data: &T) -> ApiResult<()> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(errors) => Err(ApiError::BadRequest {
            message: json!(errors).to_string(),
        }),
    }
}
