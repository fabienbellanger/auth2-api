//! CLI error module

#![allow(dead_code)]

/// CLI error code
#[derive(Debug, Clone, PartialEq)]
pub enum CliErrorCode {
    /// Invalid arguments
    InvalidArguments,
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
