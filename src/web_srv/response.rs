use serde::{Deserialize, Serialize};
use actix_web::{Error, HttpResponse};
use actix_web::web;


#[derive(Serialize, Deserialize)]
pub struct JSONStatusResponse {
    pub status: String,
}

/*pub trait Response {
    fn to_http_response2(&self) -> HttpResponse;
    /*fn to_http_response(&self) -> HttpResponse;
    fn to_ok(&self) -> Result<HttpResponse, Error> ;*/
    fn test(&self);
    fn get_json(&self) -> dyn Response;
}
pub trait JSONResponse {
}


impl<T> Response for T where T: JSONResponse {
    fn test(&self) {
        println!("test");
    }

    fn to_http_response2(&self) -> HttpResponse {
        //HttpResponse::Ok().json(self)
        //HttpResponse::Ok().json(self.get_json())
        HttpResponse::Ok().body("lol")
    }

    fn get_json(&self) -> dyn Response {
        self
    }
}

impl JSONResponse for JSONStatusResponse {
    
}*/

impl JSONStatusResponse {
    pub fn new(status: &str) -> JSONStatusResponse {
        JSONStatusResponse {
            status: status.to_string(),
        }
    }

    pub fn new_ok() -> JSONStatusResponse {
        JSONStatusResponse {
            status: "OK".to_string(),
        }
    }

    pub fn new_error(error: &str) -> JSONStatusResponse {
        JSONStatusResponse::new(error)
    }

    pub fn to_http_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }

    pub fn to_ok(&self) -> Result<HttpResponse, Error> {
        Ok(self.to_http_response())
    }
}