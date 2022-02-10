use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};

use serde::{Deserialize, Serialize};

use crate::db;
use crate::web_srv::response::JSONResponse;
use crate::db::db_sqlite::users::UserInfo;

#[derive(Deserialize)]
struct LoginJSONIn {
    username: String,
    password: String,
}

#[get("/")]
async fn index(id: Identity, _session: Session) -> Result<HttpResponse, Error> {
    /*if let Some(user) = session.get::<LoginJSONOut>("user").unwrap() {
        println!("SESSION value: {}", user.status);
    }*/

    let user = get_this_user(&id);

    // access request identity
    if user.is_none() {
        JSONResponse::new_error("Not logged in").to_ok()
    } else {
        JSONResponse::UserInfo(user.unwrap()).to_ok()
    }
}

pub fn validate_identity(id: &Identity) -> bool {
    // Returns true if there is a login identity, false otherwise

    if let Some(id) = id.identity() {
        return db::new().is_user(&id); // Make sure user exists in DB, TODO: add check for is_login, valid_session?
    }

    false
}

pub fn get_this_user(id: &Identity) -> Option<UserInfo> {
    // Gets the user profile of the current logged in user

    if let Some(id) = id.identity() {
        return db::new().get_user_by_username(&id)
    }

    None
}

pub fn does_this_user_have_permission(id: &Identity, permission: &str) -> bool {
    // Return true if user has this permission
    if let Some(user) = get_this_user(&id) {
        // * has all permissions
        if user.group.permissions == "*" {
            return true;
        }
        if user.group.permissions.contains(permission) {
            return true;
        }
    }

    false
}

fn get_login_id(json_login: &web::Json<LoginJSONIn>) -> i64 {
    // Checks if the provided login credentials are valid
    db::new().is_user_login(&json_login.username, &json_login.password)
}

#[post("/login/")]
async fn login(id: Identity, json_login: web::Json<LoginJSONIn>, session: Session) -> Result<HttpResponse, Error> {
    /*
        Logs in the user (if possible), using json_login, will set identity and session on success
    */

    // Don't bother if already logged in
    if validate_identity(&id) {
        return JSONResponse::new_error("Already logged in").to_ok();
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
        return JSONResponse::new_ok().to_ok()
    }

    JSONResponse::new_error("Bad login").to_ok()
}

#[get("/logout/")]
async fn logout(id: Identity, session: Session) -> Result<HttpResponse, Error> {
    if validate_identity(&id) {
        // Forget identity and clear session
        id.forget();
        session.clear();
        return JSONResponse::new_ok().to_ok()
    }
    JSONResponse::new_error("Not logged in").to_ok()
}
