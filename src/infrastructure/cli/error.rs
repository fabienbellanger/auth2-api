//! CLI error module

#![allow(dead_code)]

use crate::config::ConfigError;
use crate::infrastructure::api::response::ApiError;
use thiserror::Error;

/// CLI error code
#[derive(Debug, Clone, PartialEq, Error)]
pub enum CliError {
    /// Invalid arguments
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),

    /// Config error
    #[error("Config error: {0}")]
    ConfigError(String),

    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl From<ApiError> for CliError {
    fn from(err: ApiError) -> Self {
        Self::ServerError(err.to_string())
    }
}

/// Config error
impl From<ConfigError> for CliError {
    fn from(value: ConfigError) -> Self {
        Self::ConfigError(value.to_string())
    }
}
