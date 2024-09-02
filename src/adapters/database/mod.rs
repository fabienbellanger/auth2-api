//! Database adapters

use crate::config::Config;
use async_trait::async_trait;
use thiserror::Error;

pub mod mysql;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DatabaseError {
    #[error("Database pool creation error: {0}")]
    CreationPoolError(String),

    #[error("Database auto migration error: {0}")]
    AutoMigrationError(String),
}

#[async_trait]
pub trait GenericDb {
    type Db;

    async fn new(config: &Config) -> Result<Self::Db, DatabaseError>;
}
