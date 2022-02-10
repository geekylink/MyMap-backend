use sqlite::{State, Value};


use crate::db::*;

pub mod create_tables;
pub mod locations;
pub mod files;
pub mod users;
pub mod user_groups;

pub struct DbSqlite {
    connection: sqlite::Connection,
    files_table: String,
    locations_table: String,
    users_table: String,
    user_groups_table: String,
}

impl DbSqlite {
    pub fn new(db_name: &str) -> DbSqlite {
        let db = DbSqlite {
            connection: sqlite::open(db_name).unwrap(),

            // Tables
            files_table: String::from(DEFAULT_FILES_TABLE),
            locations_table: String::from(DEFAULT_LOCATIONS_TABLE),
            users_table: String::from(DEFAULT_USERS_TABLE),
            user_groups_table:String::from(DEFAULT_USER_GROUPS_TABLE),
        };

        db.first_run();
        db
    }

    pub fn first_run(&self) {
        /* 
         * Ensure Database is ready to use 
         * Insert inital data
         */
        self.create_tables();
        self.init_user_groups();
    }

    pub fn get_last_insert_rowid(&self) -> i64 {
        // Helper to get the last inserted row id

        let mut statement = self
            .connection
            .prepare("select last_insert_rowid();")
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            return statement.read::<i64>(0).unwrap();
        }

        -1
    }

    pub fn prepare_sqlite_command(&self, command: &String, values: Option<&[Value]>) -> sqlite::Cursor {
        // Helper to safely prepare an sqlite call
        
        let mut cursor = self
            .connection
            .prepare(command)
            .unwrap()
            .into_cursor();

        // Bind values if there are any
        if !values.is_none() {
            cursor
                .bind(values.unwrap())
                .unwrap();
        }

        cursor
    }

    pub fn cursor_has_row(&self, cursor: &mut sqlite::Cursor) -> bool {
        // Helper checks if cursor has a row
        if let Some(_) = cursor.next().unwrap() {
            return true;
        }

        false
    }

    pub fn cursor_insert_and_get_row_id(&self, cursor: &mut sqlite::Cursor) -> i64 {
        // Helper to execute an insertion and fetch the row id
        cursor.next().unwrap();
        self.get_last_insert_rowid()
    }

    pub fn cursor_row_to_string(&self, cursor: &mut sqlite::Cursor) -> Option<String> {
        // Helper to return a string from row (if possible)

        if let Some(row) = cursor.next().unwrap() {
            if row[0].as_string().is_none() {
                return None;
            }

            return Some(row[0].as_string().unwrap().to_string());
        }

        None
    }

    pub fn cursor_row_to_int(&self, cursor: &mut sqlite::Cursor) -> Option<i64> {
        // Helper to return a string from row (if possible)
        
        if let Some(row) = cursor.next().unwrap() {
            if row[0].as_integer().is_none() {
                return None;
            }

            return Some(row[0].as_integer().unwrap());
        }

        None
    }
}
