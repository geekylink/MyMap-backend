use actix_web::{get, post, web, Error, HttpResponse};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

use crate::db::locations::LocationData;
use crate::web_srv;
use crate::web_srv::AppState;
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
    state: web::Data<AppState>,
) -> actix_web::Result<web::Json<JSONGetLocationFilesResp>> {
    println!("getting location {}", json.id);
    let filenames = state.db.get_location_filenames(json.id).await;
    //let filenames: Vec<String> = Vec::new();

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
async fn save_location(id: Identity, state: web::Data<AppState>, json: web::Json<JSONSaveLocationData>) -> Result<HttpResponse, Error> {

    // Permission check
    if !web_srv::user::login::does_this_user_have_permission(&id, &state, "addLocation").await {
        return JSONResponse::new_error("you do not have permission").to_ok();
    }

    println!("/saveLocation/ :: {}: {}, {}, {}", json.label, json.lat, json.lon, json.location_type);

    let username = web_srv::user::login::get_this_username(&id).unwrap();
    let user_id = state.db.get_user_id(&username).await;
    let id = state.db.get_location_id(&json.label, json.lat, json.lon, &json.location_type, user_id).await;
    println!("added location");

    //let id = -1;

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
async fn get_all_locations(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(JSONGetLocationsResp {
        status: String::from("OK"),
        locations: state.db.get_all_locations().await
    }))
}
