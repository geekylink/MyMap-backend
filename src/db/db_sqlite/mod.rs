use sqlite::State;

use crate::db::*;

pub mod create_tables;
pub mod locations;
pub mod files;
pub mod users;

pub struct DbSqlite {
    connection: sqlite::Connection,
    files_table: String,
    locations_table: String,
    users_table: String,
}

impl DbSqlite {
    pub fn new(db_name: &str) -> DbSqlite {
        let db = DbSqlite {
            connection: sqlite::open(db_name).unwrap(),

            // Tables
            files_table: String::from(DEFAULT_FILES_TABLE),
            locations_table: String::from(DEFAULT_LOCATIONS_TABLE),
            users_table: String::from(DEFAULT_USERS_TABLE),
        };

        db.first_run();
        db
    }

    pub fn first_run(&self) {
        /* Ensure Database is ready to use */
        self.create_tables();
        // TODO: Insert anything into tables?
    }

    pub fn get_last_insert_rowid(&self) -> i64 {
        // Gets the last inserted row id

        let mut statement = self
            .connection
            .prepare("select last_insert_rowid();")
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            return statement.read::<i64>(0).unwrap();
        }

        -1
    }
}
