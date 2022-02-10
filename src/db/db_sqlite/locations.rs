use sqlite::Value;
use serde::Serialize;

use crate::db::db_sqlite::DbSqlite;

// Location Data stored in the locations table
#[derive(Serialize)]
pub struct LocationData {
    id: i64,
    label: String,
    lat: f64,
    lon: f64,
    location_type: String,
}

impl DbSqlite {
    pub fn add_location(&self, label: &String, lat: f64, lon: f64, location_type: &String) -> i64 {
        // Adds a new location and returns its locationID

        let mut cursor = self.connection
            .prepare(format!(
                "INSERT INTO {} (label, lat, lon, type) 
                        VALUES  (?, ?, ?, ?);",
                self.locations_table,
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(label.to_string()), 
                        Value::Float(lat), 
                        Value::Float(lon),
                        Value::String(location_type.to_string()),
                    ])
                    .unwrap();

        
        cursor.next().unwrap();

        self.get_last_insert_rowid()
    }

    fn find_location_ids(&self, label: &String, lat: f64, lon: f64, location_type: &String) -> Vec<i64> {
        // Returns a vector of location_ids that match (label, lat, lon)
        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT locationId FROM {} WHERE label=? AND lat=? AND lon=? AND type=?",
                self.locations_table,
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(label.to_string()), 
                        Value::Float(lat), 
                        Value::Float(lon),
                        Value::String(location_type.to_string()), 
                    ])
                    .unwrap();

        // We don't want multiple locations with the same lable, lat, & lon but there could be
        let mut keys = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            keys.push(row[0].as_integer().unwrap());
        }

        keys
    }

    pub fn get_location_id(&self, label: &String, lat: f64, lon: f64, location_type: &String,) -> i64 {
        // Gets a single location_id of location if it exists, otherwise creates a new location to return

        let res = self.find_location_ids(label, lat, lon, location_type);

        if res.len() == 0 {
            // If not found return a new location
            return self.add_location(label, lat, lon, location_type);
        } else if res.len() > 1 {
            println!("WARNING should not have multiple locations with the same label,lat,lon");
        }

        res[0]
    }

    pub fn get_all_locations(&self) -> Vec<LocationData> {
        // Gets all locations in the database

        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT locationId, label, lat, lon, type FROM {}",
                self.locations_table,
            ))
            .unwrap()
            .into_cursor();

        let mut locations = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            let id =   row[0].as_integer().unwrap();
            let label =         row[1].as_string().unwrap().to_string();
            let lat =           row[2].as_float().unwrap();
            let lon =           row[3].as_float().unwrap();
            let location_type = row[4].as_string().unwrap().to_string();

            locations.push(LocationData{id, label, lat, lon, location_type});
        }

        locations
    }
}