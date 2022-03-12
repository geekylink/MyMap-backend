use serde::Serialize;

use crate::db::MapDB;

#[derive(Serialize)]
pub struct FileInfo {
    pub id: i64,
    pub filename: String,
    pub title: String,
    pub description: String,
    pub location_id: i64,
    pub owner_id: i64,
}

impl MapDB { 
    pub async fn add_file(&self, location_id: i64, filename: &String, title: &String, description: &String, owner_id: i64) -> i64 {
        sqlx::query("INSERT INTO files 
                                    (location_id, filename, title, description, owner_id)  
                            VALUES  (?, ?, ?, ?, ?);")
                .bind(location_id)
                .bind(&filename)
                .bind(&title)
                .bind(&description)
                .bind(owner_id)
                .execute(&self.pool)
                .await
                .expect("Inserting new user into db")
                .last_insert_rowid()
    }

    pub async fn get_location_filenames(&self, location_id: i64) -> Vec<String> {
        let rows: Vec<(String,)> = 
            sqlx::query_as("SELECT filename FROM files WHERE location_id=?")
                .bind(location_id)
                .fetch_all(&self.pool)
                .await.ok().unwrap_or(Vec::new());

        let mut filenames = Vec::new();

        for i in 0..rows.len() {
            println!("{}", rows[i].0);
            filenames.push(rows[i].0.to_string());
        }

        filenames
    }

    pub async fn get_file(&self, filename: &String) -> FileInfo {
        sqlx::query_as!(FileInfo,
                        "SELECT * FROM files
                         WHERE filename=?",
                        filename)
                    .fetch_one(&self.pool)
                    .await.ok().expect(&format!("Could not find file '{}'", filename))
    }
}
