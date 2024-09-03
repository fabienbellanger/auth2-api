//! CLI error module

#![allow(dead_code)]

use crate::infrastructure::api::response::ApiError;

/// CLI error code
#[derive(Debug, Clone, PartialEq)]
pub enum CliErrorCode {
    /// Invalid arguments
    InvalidArguments,

    /// Server error
    ServerError,
}

/// CLI error
#[derive(Debug, Clone, PartialEq)]
pub struct CliError {
    pub code: CliErrorCode,
    pub message: String,
}

impl CliError {
    /// Create a new CLI error
    pub fn new(code: CliErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

impl From<ApiError> for CliError {
    fn from(err: ApiError) -> Self {
        Self::new(CliErrorCode::ServerError, &err.to_string())
    }
}
