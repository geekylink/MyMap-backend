use sqlite::State;

use crate::db::*;

pub mod create_tables;
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

    pub fn add_file(
        &self,
        location_id: i64,
        filename: &String,
        title: &String,
        description: &String,
    ) {
        /*
         * Insert filename into database along with metadata & location id
         */
        self.connection
            .execute(format!(
                "INSERT INTO {} (locationId, filename, title, description) 
                                     VALUES ({}, '{}', '{}', '{}');",
                self.files_table, location_id, filename, title, description
            ))
            .unwrap();
    }

    pub fn get_location_files(&self, location_id: i64) -> Vec<String> {
        //let filenames = vec!(String::from(""));
        let mut filenames: Vec<String> = vec![];

        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT filename FROM {} WHERE locationId={}",
                self.files_table, location_id
            ))
            .unwrap();

        // We don't want multiple locations with the same lable, lat, & lon but there could be
        while let State::Row = statement.next().unwrap() {
            filenames.push(statement.read::<String>(0).unwrap());
        }

        filenames
    }

    pub fn add_location(&self, label: &String, lat: f64, lon: f64) -> i64 {
        // Adds a new location and returns its locationID

        self.connection
            .execute(format!(
                "INSERT INTO {} (label, lat, lon) 
                                     VALUES ('{}', {}, {});
                                     select last_insert_rowid();",
                self.locations_table, label, lat, lon
            ))
            .unwrap();

        let mut statement = self
            .connection
            .prepare("select last_insert_rowid();")
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            return statement.read::<i64>(0).unwrap();
        }

        -1
    }

    fn find_location_ids(&self, label: &String, lat: f64, lon: f64) -> Vec<i64> {
        // Returns a vector of location_ids
        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT locationId FROM {} WHERE label='{}' AND lat={} AND lon={}",
                self.locations_table, label, lat, lon
            ))
            .unwrap();

        // We don't want multiple locations with the same lable, lat, & lon but there could be
        let mut keys = Vec::new();
        while let State::Row = statement.next().unwrap() {
            keys.push(statement.read::<i64>(0).unwrap());
        }

        keys
    }

    pub fn get_location_id(&self, label: &String, lat: f64, lon: f64) -> i64 {
        // Gets a single location_id of location if it exists, otherwise creates a new location to return

        let res = self.find_location_ids(label, lat, lon);

        if res.len() == 0 {
            // If not found return a new location
            return self.add_location(label, lat, lon);
        } else if res.len() > 1 {
            println!("WARNING should not have multiple locations with the same label,lat,lon");
        }

        res[0]
    }
}
