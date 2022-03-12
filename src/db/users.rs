use serde::Serialize; 
use chrono::Utc;

use crate::db::MapDB;
use crate::db::crypto::DbCrypto;
use crate::db::user_groups::UserGroupInfo;

/*#[derive(Serialize)]
pub struct UserRow {
    pub id:             i64,  
    pub username:       String,
    pub password:       String,
    pub salt:           String,
    pub group_id:       i64,
}*/

// User profile info, don't wish to return all info from row
#[derive(Serialize)]
pub struct UserInfo {
    pub username:       String,
    pub group:          UserGroupInfo,
}

impl MapDB { 

    pub async fn new_user(&self, username: &str, group_id: i64) -> Option<UserInfo> {
        if group_id == -1 {
            return None;
        }

        Some(UserInfo {
            username: username.to_string(),
            group: self.get_user_group_by_id(group_id).await.expect("Invalid group on user")
        })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Option<UserInfo> {
        let group_id: i64 = 
            sqlx::query_as("SELECT group_id 
                            FROM users
                            WHERE username=?;")
                        .bind(&username)
                        .fetch_one(&self.pool)
                        .await.ok().unwrap_or((-1,)).0;
        self.new_user(&username, group_id).await
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Option<UserInfo> {
        let row: (i64,String) =  
            sqlx::query_as("SELECT group_id, username 
                            FROM users
                            WHERE id=?;")
                        .bind(user_id)
                        .fetch_one(&self.pool)
                        .await.ok().unwrap_or((-1,"".to_string()));
        let group_id = row.0;
        let username = row.1;
        println!("fetched: {}, {}", username, group_id);

        self.new_user(&username, group_id).await
    }

    pub async fn get_all_users(&self) -> Vec<UserInfo> {
        let rows: Vec<(i64, String)> =
            sqlx::query_as("SELECT group_id, username FROM users")
                    .fetch_all(&self.pool)
                    .await.ok().unwrap_or(Vec::new());

        let mut users = Vec::new();

        // TODO: Could get all user groups first and pass in just the group itself
        //       Would then save a db call per user
        for i in 0..rows.len() {
            let group_id = rows[i].0;
            let username = rows[i].1.to_string();

            users.push(self.new_user(&username, group_id).await.unwrap());
        }
        
        users
    }

    pub async fn get_user_id(&self, username: &str) -> i64 {
        let row: (i64,) = sqlx::query_as("SELECT id 
                                            FROM users 
                                            WHERE username=?;")
                .bind(&username)
                .fetch_one(&self.pool)
                .await.ok().expect(&format!("Could not query username '{}'", username));

        row.0
    }

    // Returns the salt of the username from the database, or "" if none
    async fn get_user_salt(&self, username: &str) -> String {
        let row: (String,) = sqlx::query_as("SELECT salt 
                                            FROM users 
                                            WHERE username=?;")
                .bind(&username)
                .fetch_one(&self.pool)
                .await.ok().unwrap_or(("".to_string(),));

        row.0
    }

    // Returns the totp secret of the username from the database, or "" if none
    async fn get_user_totp_secret(&self, username: &str) -> String {
        let row: (String,) = sqlx::query_as("SELECT totp_secret 
                                            FROM users 
                                            WHERE username=?;")
                .bind(&username)
                .fetch_one(&self.pool)
                .await.ok().unwrap_or(("".to_string(),));

        row.0
    }

    // Adds a new user to the database, 
    // Computes a salt & hash of provided password to store
    // Returns the user_id and base64 encoding of a QR of the totp_secret
    // Can only retrieve totp_secret through this, hence one time only
    pub async fn add_user(&self, username: &str, password: &str) -> (i64, String) {
        let salt        = DbCrypto::gen_rand_salt();
        let password    = DbCrypto::password_to_hash(&password, &salt);
        let totp_secret = DbCrypto::gen_rand_secret();
        let qr_code     = DbCrypto::gen_totp_qr(username, &totp_secret);
        let guest_id = self.get_user_group_id_guest().await; // New users default to guest

        let user_id = sqlx::query("INSERT INTO users 
                                    (
                                        username, password, email, salt, group_id,
                                        totp_secret, totp_verified, email_verified,
                                        registered_date, last_login_date, last_active_date 
                                    ) 
                            VALUES  (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);")
                .bind(&username)
                .bind(&password)
                .bind("")
                .bind(&salt)
                .bind(guest_id)
                .bind(&totp_secret)
                .bind(false)
                .bind(false)
                .bind(Utc::now().timestamp())
                .bind(-1)
                .bind(-1)
                .execute(&self.pool)
                .await
                .expect("Inserting new user into db")
                .last_insert_rowid();

        (user_id, qr_code)
    }

    pub async fn add_user_to_group(&self, user_id: i64, group_id: i64) {
        sqlx::query("UPDATE users 
                            SET group_id=?
                            WHERE id=?")
                .bind(group_id)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .expect("Updating user group of user in db");
    }

    pub async fn is_user(&self, username: &str) -> bool {
        !self.get_user_by_username(username).await.is_none()
    }

    // Is this a valid TOTP code for this username?
    pub async fn is_user_totp(&self, username: &str, totp: &str) -> bool {
        let totp_secret = self.get_user_totp_secret(username).await;
        DbCrypto::is_valid_totp(&totp_secret, totp)
    }

    // Has this user verified their TOTP code since initial registation?
    pub async fn is_user_totp_verified(&self, username: &str) -> bool {
        let row: (bool,) = sqlx::query_as("SELECT totp_verified 
                                            FROM users 
                                            WHERE username=?;")
                .bind(&username)
                .fetch_one(&self.pool)
                .await.ok().unwrap_or((false,));

        row.0
    }

    // Checks if the provided credentials match a user
    // Returns -1 on failed login, or user id on success
    pub async fn is_user_login(&self, username: &str, password: &str, totp: &str) -> i64 {
        if !self.is_user_totp(username, totp).await {
            return -1;
        }

        let salt = self.get_user_salt(username).await;

        // Save us an SQL query if we didn't get a salt and just return -1
        if salt.eq("") {
            return -1;
        }

        let password = DbCrypto::password_to_hash(&password, &salt);

        let row: (i64,) = sqlx::query_as("SELECT id 
                                            FROM users 
                                            WHERE username=? AND password=?;")
                .bind(&username)
                .bind(&password)
                .fetch_one(&self.pool)
                .await.ok().unwrap_or((-1,));

        row.0
    }

    // Updates the database to record that this user has verified their TOTP
    pub async fn verified_totp(&self, username: &str) {
        sqlx::query("UPDATE users 
                            SET totp_verified=?
                            WHERE username=?")
                .bind(true)
                .bind(&username)
                .execute(&self.pool)
                .await
                .expect("Updating totp_verified of user in db");
    }
}