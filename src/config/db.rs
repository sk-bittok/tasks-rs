use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};

use crate::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub(crate) url: String,
    pub(crate) connect_timeout: u64,
    pub(crate) idle_timeout: u64,
    pub(crate) max_connections: u32,
    pub(crate) min_connections: u32,
}

impl DatabaseConfig {
    /// Returns the connection pool of this [`DatabaseConfig`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// * Database connection error
    pub fn connection_pool(&self) -> Result<PgPool, crate::Error> {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .acquire_timeout(Duration::from_secs(self.connect_timeout))
            .connect_lazy(&self.url)?;

        Ok(pool)
    }

    /// Returns the migrate of this [`DatabaseConfig`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// * Database does not exist
    /// * Database connection error
    ///  *File IO Errors
    pub async fn migrate(&self) -> Result<(), Error> {
        let migrator = self.migrator().await?;
        let pool = self.connection_pool()?;

        migrator.run(&pool).await.map_err(Into::into)
    }

    pub async fn migrator(&self) -> Result<Migrator, Error> {
        let base_dir = std::env::current_dir()?;
        let migrations_dir = base_dir.join("migrations");

        Migrator::new(migrations_dir).await.map_err(Into::into)
    }

    pub async fn truncate(&self) -> Result<(), Error> {
        let migrator = self.migrator().await?;
        let migrations = migrator.iter().count() as i64;

        migrator
            .undo(&self.connection_pool()?, migrations)
            .await
            .map_err(Into::into)
    }

    pub async fn recreate(&self) -> Result<(), Error> {
        self.truncate().await?;

        self.migrate().await
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
