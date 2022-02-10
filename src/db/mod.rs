pub mod crypto;
pub mod db_sqlite;

use self::db_sqlite::DbSqlite;

static DEFAULT_SQLITE_NAME: &str = "test.db";

static DEFAULT_FILES_TABLE: &str = "files";
static DEFAULT_LOCATIONS_TABLE: &str = "locations";
static DEFAULT_USERS_TABLE: &str = "users";
static DEFAULT_USER_GROUPS_TABLE: &str = "user_groups";

pub fn new_from(db_name: &str) -> DbSqlite {
    // Default (and only for a while probably) is to create a sqlite
    DbSqlite::new(db_name)
}

pub fn new() -> DbSqlite {
    new_from(DEFAULT_SQLITE_NAME)
}
