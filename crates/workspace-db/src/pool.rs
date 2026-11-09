use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::info;

use crate::error::DbError;

#[derive(Clone)]
pub struct WorkspaceDb {
    pool: SqlitePool,
}

impl WorkspaceDb {
    /// Connects to a SQLite database and automatically runs pending migrations.
    pub async fn connect_sqlite(database_url: &str) -> Result<Self, DbError> {
        info!("Connecting to SQLite database at: {}", database_url);

        let options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        // Run migrations automatically
        sqlx::migrate!("./migrations").run(&pool).await?;
        info!("Database migrations applied successfully.");

        Ok(Self { pool })
    }

    /// Connects to an in-memory SQLite database for unit and integration testing.
    pub async fn connect_in_memory() -> Result<Self, DbError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
