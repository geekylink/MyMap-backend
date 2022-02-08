use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};

use futures_util::TryStreamExt as _;
use serde::Serialize;

use uuid::Uuid;

use std::io::Write;
use std::str;

use crate::db;
use crate::web_srv::user;

async fn get_multipart_field(mut field: actix_multipart::Field) -> String {
    // Returns field value from multipart form

    let mut s = String::from("");

    while let Some(chunk) = field.try_next().await.ok().unwrap() {
        s = format!(
            "{:?}",
            match str::from_utf8(&chunk) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            }
        );
    }

    if s.len() < 3 {
        return s;
    }

    String::from(&s[1..s.len() - 1]) // Strip trailing quotes, "...", TODO: find a better way
}

async fn save_multipart_field(
    mut field: actix_multipart::Field,
    filename: &str,
) -> Result<bool, Error> {
    // Saves a file stored in this field of a multipart form

    let content_disposition = field
        .content_disposition()
        .ok_or_else(|| HttpResponse::BadRequest().finish())?;

    let original_filename = content_disposition.get_filename().unwrap_or("");

    let filepath = format!("./www/build/img/tmp/{}", filename);
    println!("Saving file '{}' to: {}", original_filename, filepath);

    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath)).await?;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.try_next().await? {
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
    }

    return Ok(true);
}

// Response for saving a new file
#[derive(Serialize)]
pub struct JSONSaveFileResp {
    status: String,
    filename: String,
}

impl JSONSaveFileResp {
    pub fn new(status: &str, filename: &str) -> JSONSaveFileResp {
        JSONSaveFileResp {
            status: status.to_string(),
            filename: filename.to_string(),
        }
    }

    pub fn new_error(error: &str) -> JSONSaveFileResp {
        JSONSaveFileResp::new(error, "")
    }

    pub fn to_http_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }

    pub fn to_ok(&self) -> Result<HttpResponse, Error> {
        Ok(self.to_http_response())
    }
}

#[post("/photo/{location_id}/")]
pub async fn photo(
    web::Path(location_id): web::Path<i64>,
    mut payload: Multipart,
    id: Identity,
    _session: Session,
) -> Result<HttpResponse, Error> {
    // TODO: Add verification to determine is photo
    // TODO: limit file size
    // TODO: only save file if title is also provided

    if !user::login::validate_identity(&id) {
        return JSONSaveFileResp::new_error("error: not logged in").to_ok();
    }

    // Uploads a file and saves its name, location id, and metadata to db
    println!("Location: {}", location_id);

    let save_name = Uuid::new_v4().to_string() + &".jpg".to_string(); // Generate a UUID for the filename for safe storage
    let mut title = String::from("");
    let mut description = String::from("");
    let mut error = "";

    // iterate over multipart stream
    while let Some(field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let name = content_disposition.get_name().unwrap_or("");

        if name == "file" {
            let save_result = save_multipart_field(field, &save_name)
                .await
                .ok()
                .unwrap_or(false);
            if save_result {
                println!("File saved successfully");
            } else {
                error = "Error: File could not be saved.";
                break;
            }
        } else if name == "title" {
            title = get_multipart_field(field).await;
            println!("filename title '{}'", title);
        } else if name == "description" {
            description = get_multipart_field(field).await;
        }
    }

    let filename: &str;
    let status: &str;

    // Save filename & data into database
    if error.eq("") {
        if title != "" {
            println!("Inserting into database");
            println!("{}:\n{}", title, description);
            // TODO: BLOCKING OPERATION
            let db = db::new();
            db.add_file(location_id, &save_name, &title, &description);

            filename = &save_name;
            status = "OK";
        } else {
            status = "Error: No title provided for file";
            filename = "";
        }
    } else {
        status = error;
        filename = "";
    }

    return JSONSaveFileResp::new(status, filename).to_ok();
}
