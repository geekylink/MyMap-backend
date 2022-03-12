use serde::Serialize;
use actix_web::{Error, HttpResponse};

use crate::db::users::UserInfo;

// Simple status message to return
#[derive(Serialize)]
pub struct StatusMsg {
    pub status: String,
}

// Error messages to return
#[derive(Serialize)]
pub struct ErrorMsg {
    pub error: String,
}



// Various JSON responses to return to the web client
pub enum JSONResponse {
    StatusMsg(StatusMsg),
    ErrorMsg(ErrorMsg),
    UserInfo(UserInfo),
}


impl JSONResponse {
    pub fn new_ok() -> JSONResponse {
        // Simple {status: "OK"} message
        JSONResponse::StatusMsg(StatusMsg {
            status: "OK".to_string(),
        })
    }

    pub fn new_status(status: &str) -> JSONResponse {
        // Returns {status: "status message"}
        JSONResponse::StatusMsg(StatusMsg {
            status: status.to_string(),
        })
    }

    pub fn new_error(error: &str) -> JSONResponse {
        // Error {error: "error message"}
        JSONResponse::ErrorMsg(ErrorMsg {
            error: error.to_string(),
        })
    }

    pub fn to_http_response(&self) -> HttpResponse {
        // Converts the JSONResponse to a HttpResponse to return to client

        match self {
            JSONResponse::ErrorMsg(error_info) => HttpResponse::Ok().json(error_info),
            JSONResponse::StatusMsg(status) => HttpResponse::Ok().json(status),
            JSONResponse::UserInfo(user_info) => HttpResponse::Ok().json(user_info),
        }
    }

    pub fn to_ok(&self) -> Result<HttpResponse, Error> {
        // Simple wrapper around HttpResponse to an Ok()
        Ok(self.to_http_response())
    }
}