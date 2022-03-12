use actix_web::{get, post, web, Error, HttpResponse};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

use crate::web_srv::AppState;
use crate::web_srv::user;
use crate::web_srv::response::JSONResponse;
use crate::db::comments::CommentDataForClient;

#[derive(Deserialize)]
struct AddCommentPost {
    comment: String,
    file_id: Option<i64>,
    location_id: Option<i64>,
    reply_to_id: Option<i64>,
}

#[derive(Deserialize)]
struct EditCommentPost {
    id: i64,
    comment: String,
}

#[derive(Serialize)]
struct AddCommentResp {
    status: String,
    id: i64,
}

#[derive(Serialize)]
struct EditCommentResp {
    status: String,
}


#[derive(Serialize)]
struct GetCommentsResp {
    status: String,
    comments: Vec<CommentDataForClient>,
}

#[post("/addComment/")]
async fn add_comment(id: Identity, state: web::Data<AppState>, json: web::Json<AddCommentPost>) -> Result<HttpResponse, Error> {
    // Permission check
    if !user::login::does_this_user_have_permission(&id, &state, "addComment").await {
        return JSONResponse::new_error("you do not have permission").to_ok();
    }

    let user_id = user::login::get_this_user_id(&id, &state).await;

    let file_id     = json.file_id.unwrap_or(-1);
    let location_id = json.location_id.unwrap_or(-1);
    let reply_to_id = json.reply_to_id.unwrap_or(-1);

    if file_id == -1 && location_id == -1 {
        return JSONResponse::new_error("Comment must be posted on either a file or location").to_ok();
    }
    else if file_id != -1 && location_id != -1 {
        return JSONResponse::new_error("Comment cannot be on both a file and location").to_ok();
    }

    // TODO: verify location/file before adding comment!

    let comment_id = if reply_to_id == -1 {
        state.db.add_comment(&json.comment, location_id, file_id, user_id).await
    }
    else {
        state.db.add_reply(&json.comment, location_id, file_id, user_id, reply_to_id).await
    };
    
    Ok(HttpResponse::Ok().json(AddCommentResp {
        status:         "OK".to_string(),
        id:             comment_id,   
    }))
}

#[post("/editComment/")]
async fn edit_comment(id: Identity, state: web::Data<AppState>, json: web::Json<EditCommentPost>) -> Result<HttpResponse, Error> {
    // Permission check
    if !user::login::does_this_user_have_permission(&id, &state, "editComment").await {
        return JSONResponse::new_error("you do not have permission").to_ok();
    }

    let user_id = user::login::get_this_user_id(&id, &state).await;
    let comment = state.db.get_comment(json.id).await;

    if comment.is_none() {
        return JSONResponse::new_error("not a valid comment id").to_ok();
    }

    if comment.unwrap().owner_id != user_id && !user::login::does_this_user_have_permission(&id, &state, "editOtherComment").await {
        return JSONResponse::new_error("you cannot edit other user comments").to_ok();
    }

    state.db.edit_comment(json.id, &json.comment).await;
    
    Ok(HttpResponse::Ok().json(EditCommentResp {
        status:         "OK".to_string(), 
    }))
}

#[get("/getCommentsOnLocation/{location_id}/")]
async fn get_comments_on_location(web::Path(location_id): web::Path<i64>, state: web::Data<AppState>,) -> Result<HttpResponse, Error> {
    let comments = state.db.get_comments_on_location(location_id).await;

    Ok(HttpResponse::Ok().json(GetCommentsResp {
        status:         "OK".to_string(),
        comments
    }))
}

#[get("/getCommentsOnFile/{file_id}/")]
async fn get_comments_on_file(web::Path(file_id): web::Path<i64>, state: web::Data<AppState>,) -> Result<HttpResponse, Error> {
    let comments = state.db.get_comments_on_file(file_id).await;

    Ok(HttpResponse::Ok().json(GetCommentsResp {
        status:         "OK".to_string(),
        comments
    }))
}

#[get("/getReplies/{reply_to_id}/")]
async fn get_replies(web::Path(reply_to_id): web::Path<i64>, state: web::Data<AppState>,) -> Result<HttpResponse, Error> {
    let comments = state.db.get_replies(reply_to_id).await;

    Ok(HttpResponse::Ok().json(GetCommentsResp {
        status:         "OK".to_string(),
        comments
    }))
}