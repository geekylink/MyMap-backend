use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};

use serde::{Deserialize, Serialize};

use crate::db;
use crate::web_srv::response::{/*Response, JSONResponse,*/ JSONStatusResponse};

#[derive(Deserialize)]
struct LoginJSONIn {
    username: String,
    password: String,
}

#[get("/")]
async fn index(id: Identity, _session: Session) -> HttpResponse {
    /*if let Some(user) = session.get::<LoginJSONOut>("user").unwrap() {
        println!("SESSION value: {}", user.status);
    }*/

    // access request identity
    if let Some(id) = id.identity() {
        return HttpResponse::Ok().body(format!("{}", id));
    } else {
        return HttpResponse::Ok().body("".to_owned());
    }
}

fn get_login_id(json_login: &web::Json<LoginJSONIn>) -> i64 {
    let db = db::new();

    db.is_user_login(&json_login.username, &json_login.password)
}

pub fn validate_identity(id: &Identity) -> bool {
    // Returns true if there is a login identity, false otherwise

    if let Some(_) = id.identity() {
        return true;
    }

    false
}

#[post("/login/")]
async fn login(id: Identity, json_login: web::Json<LoginJSONIn>, session: Session) -> Result<HttpResponse, Error> {
    /*
        Logs in the user (if possible), using json_login, will set identity and session on success
    */

    // Don't bother if already logged in
    if validate_identity(&id) {
        return JSONStatusResponse::new_error("Already logged in").to_ok()
    }

    if get_login_id(&json_login) != -1 {
        // Remember identity and save session
        id.remember(json_login.username.to_owned());

        // TODO: Make useful session info
        /*session
            .set(
                "user",
                LoginJSONOut {
                    status: "lolk".to_string(),
                },
            )
            .ok();*/

        println!("login success");
        return JSONStatusResponse::new_ok().to_ok()
    }
    
    JSONStatusResponse::new_error("Bad login").to_ok()
}

#[get("/logout/")]
async fn logout(id: Identity, session: Session) -> Result<HttpResponse, Error> {
    let status: String;

    if validate_identity(&id) {
        // Forget identity and clear session
        id.forget();
        session.clear();
        return JSONStatusResponse::new_ok().to_ok()
    }

    JSONStatusResponse::new_error("Not logged in").to_ok()
}
