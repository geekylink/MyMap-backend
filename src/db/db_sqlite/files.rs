use sqlite::Value;

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
        let mut cursor = self.connection
            .prepare(format!(
                "INSERT INTO {} (locationId, filename, title, description) 
                                     VALUES (?, ?, ?, ?);",
                self.files_table,
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[
                    Value::Integer(location_id),
                    Value::String(filename.to_string()),
                    Value::String(title.to_string()),
                    Value::String(description.to_string()),
                ])
            .unwrap();

        cursor.next().unwrap();
    }

    pub fn get_file_info(&self, filename: &String) -> FileInfo {
        let title: String;
        let description: String;

        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT title, description FROM {} WHERE filename=?",
                self.files_table
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[Value::String(filename.to_string()),])
            .unwrap();

        if let Some(row) = cursor.next().unwrap() {
            title       = row[0].as_string().unwrap().to_string();
            description = row[1].as_string().unwrap().to_string();
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

        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT filename FROM {} WHERE locationId=?",
                self.files_table
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[Value::Integer(location_id),])
            .unwrap();

        // We don't want multiple locations with the same lable, lat, & lon but there could be
        while let Some(row) = cursor.next().unwrap() {
            filenames.push(row[0].as_string().unwrap().to_string());
        }

        filenames
    }
}