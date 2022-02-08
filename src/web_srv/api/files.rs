use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Deserialize)]
struct GetFileInfoReq {
    filename: String,
}

#[derive(Serialize)]
struct GetFileInfoResp {
    status: String,
    filename: String,
    title: String,
    description: String,
}

#[post("/getFileInfo/")]
async fn get_file_info(json: web::Json<GetFileInfoReq>) -> Result<HttpResponse, Error> {
    println!("getting file {}", &json.filename);
    let db = db::new();
    let file_info = db.get_file_info(&json.filename);

    Ok(HttpResponse::Ok().json(GetFileInfoResp {
        status:         "OK".to_string(),
        filename:       file_info.filename,
        title:          file_info.title,
        description:    file_info.description,
    }))
}