use sqlite::State;

use crate::db::db_sqlite::DbSqlite;

pub struct FileInfo {
    pub filename: String,
    pub title: String,
    pub description: String,
}

impl DbSqlite {
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

    pub fn get_file_info(&self, filename: &String) -> FileInfo {
        let title: String;
        let description: String;

        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT title, description FROM {} WHERE filename='{}'",
                self.files_table, &filename
            ))
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            title       = statement.read::<String>(0).unwrap();
            description = statement.read::<String>(1).unwrap();
        }
        else {
            title = "".to_string();
            description = "".to_string();
        }
        

        FileInfo {
            filename: filename.to_string(),
            title,
            description
        }
    }

    pub fn get_location_files(&self, location_id: i64) -> Vec<String> {
        /*
         * Get all filenames from a location_id
         * Returns vector of strings of filenames
         */
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
}