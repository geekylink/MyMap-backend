use serde::Serialize;

use crate::db::MapDB;
use chrono::Utc;

use crate::db::users::UserInfo;

// Location Data stored in the locations table
#[derive(Serialize)]
pub struct CommentData {
    pub id: i64,
    pub comment: String,
    pub file_id: i64,
    pub location_id: i64,
    pub owner_id: i64,
    pub reply_to_id: i64,
    pub posted_date: f32,
    pub last_edit_date: f32,
}

#[derive(Serialize)]
pub struct CommentDataForClient {
    pub user: UserInfo,
    pub id: i64,
    pub comment: String,
    pub reply_to_id: i64,
    pub posted_date: f32,
    pub last_edit_date: f32,
}

impl CommentData {
    // Data to return to web client
    pub async fn for_client(&self, db: &MapDB) -> CommentDataForClient {
        println!("owner: {}", self.owner_id);
        CommentDataForClient {
            user:           db.get_user_by_id(self.owner_id).await.expect("Bad owner_id on comment"),
            id:             self.id,
            comment:        self.comment.to_string(),
            reply_to_id:    self.reply_to_id,
            posted_date:    self.posted_date,
            last_edit_date: self.last_edit_date,
        }
    }
}

impl MapDB {
    pub async fn add_comment(&self, comment: &str, location_id: i64, file_id: i64, owner_id: i64) -> i64 {
        sqlx::query("INSERT INTO comments 
                                    (comment, location_id, file_id, owner_id, reply_to_id, posted_date, last_edit_date)  
                            VALUES  (?, ?, ?, ?, ?, ?, ?);")
                .bind(&comment)
                .bind(location_id)
                .bind(file_id)
                .bind(owner_id)
                .bind(-1)
                .bind(Utc::now().timestamp())
                .bind(-1)
                .execute(&self.pool)
                .await
                .expect("Inserting new comment into db")
                .last_insert_rowid()
    }

    pub async fn add_reply(&self, comment: &str, location_id: i64, file_id: i64, owner_id: i64, reply_to_id: i64) -> i64 {
        sqlx::query("INSERT INTO comments 
                                    (comment, location_id, file_id, owner_id, reply_to_id, posted_date, last_edit_date)  
                            VALUES  (?, ?, ?, ?, ?, ?, ?);")
                .bind(&comment)
                .bind(location_id)
                .bind(file_id)
                .bind(owner_id)
                .bind(reply_to_id)
                .bind(Utc::now().timestamp())
                .bind(-1)
                .execute(&self.pool)
                .await
                .expect("Inserting new reply into db")
                .last_insert_rowid()
    }

    pub async fn edit_comment(&self, comment_id: i64, comment: &str) {
        sqlx::query("UPDATE comments 
                            SET comment=?, last_edit_date=?
                            WHERE id=?")
                .bind(&comment)
                .bind(Utc::now().timestamp())
                .bind(comment_id)
                .execute(&self.pool)
                .await
                .expect("Updating comment in db");
    }

    async fn comment_data_for_client(&self, rows: &Vec<CommentData>) -> Vec<CommentDataForClient> {
        let mut comments = Vec::new();

        for i in 0..rows.len() {
            comments.push(rows[i].for_client(self).await);
        }

        comments
    }

    pub async fn get_comments_on_location(&self, location_id: i64) -> Vec<CommentDataForClient> {
        // Gets all top-level comments on this location
        self.comment_data_for_client(
            &sqlx::query_as!(CommentData,
                        "SELECT * FROM comments
                         WHERE file_id=-1 AND location_id=? AND reply_to_id=-1;",
                        location_id)
                    .fetch_all(&self.pool)
                    .await.ok().expect("Getting comments on location")//.unwrap_or(Vec::new())
        ).await
    }

    pub async fn get_comments_on_file(&self, file_id: i64) -> Vec<CommentDataForClient> {
        // Gets all top-level comments on this file
        self.comment_data_for_client(
            &sqlx::query_as!(CommentData,
                        "SELECT * FROM comments
                         WHERE file_id=? AND location_id=-1 AND reply_to_id=-1;",
                        file_id)
                    .fetch_all(&self.pool)
                    .await.ok().expect("Getting comments on file")//.unwrap_or(Vec::new())
        ).await
    }

    pub async fn get_comment(&self, comment_id: i64) -> Option<CommentData> {
        sqlx::query_as!(CommentData,
            "SELECT * FROM comments
             WHERE id=?;",
            comment_id)
        .fetch_one(&self.pool)
        .await.ok()
    }

    pub async fn get_replies(&self, comment_id: i64) -> Vec<CommentDataForClient> {
        self.comment_data_for_client(
            &sqlx::query_as!(CommentData,
                        "SELECT * FROM comments
                         WHERE reply_to_id=?;",
                        comment_id)
                    .fetch_all(&self.pool)
                    .await.ok().expect("Getting replies")//.unwrap_or(Vec::new())
        ).await
    }
}
