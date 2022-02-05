use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Deserialize)]
struct JSONGetLocationFilesParams {
    id: i64,
}

#[derive(Serialize)]
struct JSONGetLocationFilesResp {
    status: String,
    error: String,
    filenames: Vec<String>,
}

#[post("/getLocationFiles/")]
async fn get_location_files(
    json: web::Json<JSONGetLocationFilesParams>,
) -> actix_web::Result<web::Json<JSONGetLocationFilesResp>> {
    println!("getting location {}", json.id);
    let db = db::new();
    let filenames = db.get_location_files(json.id);

    Ok(web::Json(JSONGetLocationFilesResp {
        status: String::from("OK"),
        error: String::from(""),
        filenames,
    }))
}

#[derive(Deserialize)]
struct JSONSaveLocationData {
    label: String,
    lat: f64,
    lon: f64,
}

#[derive(Serialize)]
struct JSONSaveLocationResp {
    status: String,
    id: i64,
}

#[post("/saveLocation/")]
async fn save_location(json: web::Json<JSONSaveLocationData>) -> Result<HttpResponse, Error> {
    println!("{}: {}, {}", json.label, json.lat, json.lon);

    // TODO: BLOCKING OPERATION
    let db = db::new();
    println!("Got db");
    let id = db.get_location_id(&json.label, json.lat, json.lon);
    println!("added location");

    Ok(HttpResponse::Ok().json(JSONSaveLocationResp {
        status: String::from("OK"),
        id: id,
    }))
}
