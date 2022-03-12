use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};

use serde::{Deserialize, Serialize};

use crate::web_srv::response::JSONResponse;
use crate::db::users::UserInfo;
use crate::web_srv::AppState;

#[derive(Deserialize)]
struct LoginJSONIn {
    username:   String,
    password:   String,
    totp_code:  Option<String>,
}

#[derive(Deserialize)]
struct LoginTOTPReq {
    totp_code:  String,
}

// QR code in BASE64 String
#[derive(Serialize)]
pub struct QrResp {
    pub status: String,
    pub qr_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserClientData {
    pub totp_verified: bool,
}


pub async fn validate_identity(id: &Identity, state: &web::Data<AppState>) -> bool {
    // Returns true if there is a login identity, false otherwise

    if let Some(id) = id.identity() {
        return state.db.is_user(&id).await; // Make sure user exists in DB, TODO: add check for is_login, valid_session?
    }

    false
}

pub async fn validate_session(id: &Identity, session: &Session, state: &web::Data<AppState>) -> bool {
    if !validate_identity(id, state).await {
        return false;
    }

    // User is required to verify TOTP before they can do things
    if let Some(user) = session.get::<UserClientData>("user").unwrap() {
        return user.totp_verified;
    }

    false
}

// Gets the user profile of the current logged in user
pub async fn get_this_user(id: &Identity, state: &web::Data<AppState>) -> Option<UserInfo> {
    if let Some(id) = id.identity() {
        return state.db.get_user_by_username(&id).await
    }

    None
}

// Gets the username of the current logged in user
pub fn get_this_username(id: &Identity) -> Option<String> {
    if let Some(id) = id.identity() {
        return Some(id)
    }

    None
}

// Gets the user_id of the current logged in user
pub async fn get_this_user_id(id: &Identity, state: &web::Data<AppState>) -> i64 {
    if let Some(id) = id.identity() {
        return state.db.get_user_id(&id).await;
    }

    -1
}

// Return true if user has this permission or "*"
pub async fn does_this_user_have_permission(id: &Identity, state: &web::Data<AppState>, permission: &str) -> bool {
    if let Some(user) = get_this_user(&id, &state).await {
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

// Returns the user_id if the credentials in json_login are correct, otherwise -1
async fn get_login_id(json_login: &web::Json<LoginJSONIn>, state: &web::Data<AppState>) -> i64 {
    // Checks if the provided login credentials are valid
    if json_login.totp_code.is_none() {
        return -1;
    }

    state.db.is_user_login(
            &json_login.username, 
            &json_login.password, 
            &json_login.totp_code.as_ref().unwrap()
    ).await
}

// Set session information
fn set_session(session: &Session, totp_verified: bool) {
    session
        .set(
            "user",
            UserClientData {
                totp_verified,
            },
        )
        .ok();
}

// Returns the user profile of the currently logged in user to the client
#[get("/")]
async fn index(id: Identity, state: web::Data<AppState>, _session: Session) -> Result<HttpResponse, Error> {
    let user = get_this_user(&id, &state).await;

    // access request identity
    if user.is_none() {
        JSONResponse::new_error("Not logged in").to_ok()
    } else {
        JSONResponse::UserInfo(user.unwrap()).to_ok()
    }
}

// Returns the user profile of another user (if logged in and allowed)
#[get("/users/{username}/")]
async fn get_user(web::Path(username): web::Path<String>, id: Identity, state: web::Data<AppState>, session: Session) -> Result<HttpResponse, Error> 
{
    if !validate_session(&id, &session, &state).await {
        return JSONResponse::new_error("Must be logged in").to_ok();
    }

    let user = state.db.get_user_by_username(&username).await;

    // access request identity
    if user.is_none() {
        JSONResponse::new_error("No such user").to_ok()
    } else {
        JSONResponse::UserInfo(user.unwrap()).to_ok()
    }
}

// Returns OK if username exists, used to check if user should login/register
#[get("/isUser/{username}/")]
async fn is_user(web::Path(username): web::Path<String>, state: web::Data<AppState>) -> Result<HttpResponse, Error> 
{
    let user = state.db.get_user_by_username(&username).await;

    // Returns OK if user, otherwise error
    if user.is_none() {
        JSONResponse::new_error("No such user").to_ok()
    } else {
        JSONResponse::new_ok().to_ok()
    }
}

// Logs in the user (if possible), using json_login, will set identity and session on success
#[post("/login/")]
async fn login(id: Identity, json_login: web::Json<LoginJSONIn>, session: Session, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // Don't bother if already logged in
    if validate_identity(&id, &state).await {
        return JSONResponse::new_error("Already logged in").to_ok();
    }

    if get_login_id(&json_login, &state).await != -1 {
        // Remember identity and save session
        id.remember(json_login.username.to_owned());
        set_session(&session, state.db.is_user_totp_verified(&json_login.username).await);

        println!("login success");
        return JSONResponse::new_ok().to_ok()
    }

    JSONResponse::new_error("Bad login").to_ok()
}

// Logs out a user and clears their session (if they are actually logged in)
#[get("/logout/")]
async fn logout(id: Identity, session: Session, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if validate_identity(&id, &state).await {
        // Forget identity and clear session
        id.forget();
        session.clear();
        return JSONResponse::new_ok().to_ok()
    }
    JSONResponse::new_error("Not logged in").to_ok()
}

// Registers a new user if username does not exist and we are not currently logged in
#[post("/register/")]
async fn register(id: Identity, json_login: web::Json<LoginJSONIn>, session: Session, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if validate_identity(&id, &state).await {
        return JSONResponse::new_error("Already logged in").to_ok();
    }

    // Do not create a new user if this username exists in the db
    if state.db.is_user(&json_login.username).await {
        return JSONResponse::new_error("User already exists").to_ok();
    } 

    let res = state.db.add_user(&json_login.username, &json_login.password).await;
    let qr_code = res.1; // Only retrievable one-time during user creation

    id.remember(json_login.username.to_owned());

    set_session(&session, false);

    Ok(HttpResponse::Ok().json(QrResp {
        status:         "OK".to_string(),
        qr_code,
    }))
}

// Verifies the TOTP provided and sets the totp_verified for the user on success
// Only used once, after /register/
#[post("/totp/")]
async fn check_totp(id: Identity, json_login: web::Json<LoginTOTPReq>, state: web::Data<AppState>, session: Session) -> Result<HttpResponse, Error> {

    let username = get_this_username(&id);

    if username.is_none() {
        return JSONResponse::new_error("Not logged in").to_ok();
    }

    let username = username.unwrap();
    let totp_code = json_login.totp_code.as_ref();

    if state.db.is_user_totp(&username, &totp_code).await {
        if !state.db.is_user_totp_verified(&username).await {
            state.db.verified_totp(&username).await;
            set_session(&session, true);
        }
        return JSONResponse::new_ok().to_ok();
    }

    JSONResponse::new_error("Invalid TOTP").to_ok()
}
