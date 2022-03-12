use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::web_srv::AppState;

#[derive(Deserialize)]
struct GetFileInfoReq {
    filename: String,
}

#[derive(Serialize)]
struct GetFileInfoResp {
    status: String,
    file_id: i64,
    filename: String,
    title: String,
    description: String,
}

#[post("/getFileInfo/")]
async fn get_file_info(json: web::Json<GetFileInfoReq>, state: web::Data<AppState>,) -> Result<HttpResponse, Error> {
    println!("getting file {}", &json.filename);

    let file = state.db.get_file(&json.filename).await;

    Ok(HttpResponse::Ok().json(GetFileInfoResp {
        status:         "OK".to_string(),
        file_id:        file.id,
        filename:       file.filename,
        title:          file.title,
        description:    file.description,
    }))
}