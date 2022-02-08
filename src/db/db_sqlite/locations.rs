use sqlite::Value;

use crate::db::db_sqlite::DbSqlite;

impl DbSqlite {
    pub fn add_location(&self, label: &String, lat: f64, lon: f64) -> i64 {
        // Adds a new location and returns its locationID

        let mut cursor = self.connection
            .prepare(format!(
                "INSERT INTO {} (label, lat, lon) 
                        VALUES  (?, ?, ?);",
                self.locations_table,
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(label.to_string()), 
                        Value::Float(lat), 
                        Value::Float(lon)]).unwrap();

        
        cursor.next().unwrap();

        self.get_last_insert_rowid()
    }

    fn find_location_ids(&self, label: &String, lat: f64, lon: f64) -> Vec<i64> {
        // Returns a vector of location_ids
        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT locationId FROM {} WHERE label=? AND lat=? AND lon=?",
                self.locations_table,
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(label.to_string()), 
                        Value::Float(lat), 
                        Value::Float(lon)
                    ])
                    .unwrap();

        // We don't want multiple locations with the same lable, lat, & lon but there could be
        let mut keys = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            keys.push(row[0].as_integer().unwrap());
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