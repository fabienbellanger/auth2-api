//! MySQL adapter

use crate::adapters::database::{DatabaseError, GenericDb};
use crate::config::Config;
use async_trait::async_trait;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use std::time::Duration;

pub mod repositories;

/// MySQL database adapter
#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Arc<Pool<MySql>>,
}

#[async_trait]
impl GenericDb for Db {
    type Db = Db;

    async fn new(config: &Config) -> Result<Self::Db, DatabaseError> {
        let url = &config.database_url;
        let max_connections = config.database_max_connections;
        let min_connections = config.database_min_connections;
        let max_lifetime = config.database_max_lifetime;
        let connect_timeout = config.database_connect_timeout;
        let idle_timeout = config.database_idle_timeout;

        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .max_lifetime(Some(Duration::from_secs(max_lifetime)))
            .acquire_timeout(Duration::from_secs(connect_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .test_before_acquire(true)
            .connect(url)
            .await
            .map_err(|err| DatabaseError::CreationPoolError(err.to_string()))?;

        if config.database_auto_migration {
            info!("Run database migrations");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|err| DatabaseError::AutoMigrationError(err.to_string()))?
        }

        Ok(Self { pool: Arc::new(pool) })
    }
}
