use serde::Serialize;

use crate::db::MapDB;

// Location Data stored in the locations table
#[derive(Serialize)]
pub struct LocationData {
    pub id: i64,
    pub label: String,
    pub lat: f32,
    pub lon: f32,
    pub kind: String,
    pub owner_id: i64,
}

impl MapDB { 
    //pub async fn add_location(&self, location: &LocationData) -> i64 {
    pub async fn add_location(&self, label: &String, lat: f64, lon: f64, kind: &String, owner_id: i64) -> i64 {
        sqlx::query("INSERT INTO locations 
                                    (label, lat, lon, kind, owner_id) 
                            VALUES  (?, ?, ?, ?, ?);")
                .bind(&label)
                .bind(lat)
                .bind(lon)
                .bind(&kind)
                .bind(owner_id)
                .execute(&self.pool)
                .await
                .expect("Inserting location into db")
                .last_insert_rowid()
    }

    pub async fn get_all_locations(&self) -> Vec<LocationData> {
        sqlx::query_as!(LocationData, 
                        "SELECT * FROM locations")
                    .fetch_all(&self.pool)
                    .await.ok().unwrap()
    }

    async fn get_location_ids(&self, label: &String, lat: f64, lon: f64, location_type: &String, owner_id: i64) -> Vec<i64> {
        let rows: Vec<(i64,)> = if owner_id == -1 {
            sqlx::query_as("SELECT id FROM locations
                            WHERE label=? and lat=? and lon=? and location_type=?")
                        .bind(&label)
                        .bind(lat)
                        .bind(lon)
                        .bind(&location_type)
                        .fetch_all(&self.pool)
                        .await.ok().unwrap_or(Vec::new())
        }
        else {
            sqlx::query_as("SELECT id FROM locations
                            WHERE label=? and lat=? and lon=? and location_type=? and owner_id=?")
                        .bind(&label)
                        .bind(lat)
                        .bind(lon)
                        .bind(&location_type)
                        .bind(owner_id)
                        .fetch_all(&self.pool)
                        .await.ok().unwrap_or(Vec::new())
        };
        
        let mut ids = Vec::new();

        for i in 0..rows.len() {
            ids.push(rows[i].0);
        }

        ids
    }

    pub async fn get_location_id(&self, label: &String, lat: f64, lon: f64, kind: &String, owner_id: i64) -> i64 {
        // Returns a vector of location_ids that match (label, lat, lon, owner_id)
        // Can ignore owner_id by passing in -1
        let location_ids = self.get_location_ids(&label, lat, lon, &kind, owner_id).await;

        if owner_id != -1 && location_ids.len() > 1 {
            panic!("Too many locations returned! Each owner should only have one location with the same (label, lat, lon, type)")
        }
        else if location_ids.len() == 0 {
            return self.add_location(label, lat, lon, kind, owner_id).await;
        }

        location_ids[0]
    }
}
