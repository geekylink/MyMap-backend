use std::time::Duration;
use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};

const DEFAULT_SQLITE_NAME: &str = "db.sqlite";

// Pool settings
const POOL_TIMEOUT: Duration        = Duration::from_secs(30);
const POOL_MAX_CONNECTIONS: u32     = 4;

async fn open_sqlite_db() -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
        let database_url = format!("sqlite://{}", DEFAULT_SQLITE_NAME);
    
        let connection_options = SqliteConnectOptions::from_str(&database_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(POOL_TIMEOUT);
    
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(POOL_MAX_CONNECTIONS)
            .connect_timeout(POOL_TIMEOUT)
            .connect_with(connection_options)
            .await?;
    
        // Migrate doesn't work on sqlx=0.4.2?, not empty anyway
        sqlx::migrate!().run(&sqlite_pool).await?; // create tables, init groups, etc
    
        Ok(sqlite_pool)
}

#[tokio::main] // to build initial migrations for w/e reason, tokio version mismatch
async fn main() -> std::io::Result<()> {
    let _ = open_sqlite_db().await.expect("Failed to open db");
    return Ok(());
}
