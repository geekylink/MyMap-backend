use std::time::Duration;
use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};

pub mod comments;
pub mod crypto;
pub mod files;
pub mod locations;
pub mod users;
pub mod user_groups;

// SQLite database file
const DEFAULT_SQLITE_NAME: &str = "db.sqlite";

// Pool settings
const POOL_TIMEOUT: Duration        = Duration::from_secs(30);
const POOL_MAX_CONNECTIONS: u32     = 4;

#[derive(Clone)]
pub struct MapDB {
    pub pool: Pool<Sqlite>,
}

impl MapDB {
    pub async fn new_from(db_name: &str) -> MapDB {
        // Default (and only for a while probably) is to create a sqlite
        MapDB {
            pool: MapDB::open_sqlite_db(&db_name).await.expect("could not open db"),
        }
    }
    
    pub async fn new() -> MapDB {
        MapDB::new_from(DEFAULT_SQLITE_NAME).await
    }

    async fn open_sqlite_db(db_name: &str) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
        let database_url = format!("sqlite://{}", db_name);
    
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
}