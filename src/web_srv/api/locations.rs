use actix_web::{get, post, web, Error, HttpResponse};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::db::db_sqlite::locations::LocationData;
use crate::web_srv;
use crate::web_srv::response::JSONResponse;

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
    location_type: String,
}

#[derive(Serialize)]
struct JSONSaveLocationResp {
    status: String,
    id: i64,
}

#[post("/saveLocation/")]
async fn save_location(id: Identity, json: web::Json<JSONSaveLocationData>) -> Result<HttpResponse, Error> {

    // Permission check
    if !web_srv::user::login::does_this_user_have_permission(&id, "saveLocation") {
        return JSONResponse::new_error("you do not have permission").to_ok();
    }

    println!("/saveLocation/ :: {}: {}, {}, {}", json.label, json.lat, json.lon, json.location_type);

    // TODO: BLOCKING OPERATION
    let db = db::new();
    println!("Got db");
    let id = db.get_location_id(&json.label, json.lat, json.lon, &json.location_type);
    println!("added location");

    Ok(HttpResponse::Ok().json(JSONSaveLocationResp {
        status: String::from("OK"),
        id: id,
    }))
}

#[derive(Serialize)]
struct JSONGetLocationsResp {
    status: String,
    locations: Vec<LocationData>,
}

#[get("/getAllLocations/")]
async fn get_all_locations() -> Result<HttpResponse, Error> {

    // TODO: BLOCKING OPERATION
    let db = db::new();
    let locations = db.get_all_locations();

    Ok(HttpResponse::Ok().json(JSONGetLocationsResp {
        status: String::from("OK"),
        locations
    }))
}
