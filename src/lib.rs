pub mod adapters;
pub(crate) mod config;
pub mod domain;
pub mod infrastructure;

#[macro_use]
extern crate tracing;

/// Application name
pub const APP_NAME: &str = "Auth2 API";
