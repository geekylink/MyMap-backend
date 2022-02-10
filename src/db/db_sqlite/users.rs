use sqlite::Value;
use serde::Serialize;

use crate::db::crypto::DbCrypto;
use crate::db::db_sqlite::DbSqlite;
use crate::db::db_sqlite::user_groups::UserGroupInfo;

// User profile info 
#[derive(Serialize)]
pub struct UserInfo {
    pub id:                 i64,  
    pub username:       String,
    pub group:          UserGroupInfo,
}

impl DbSqlite {
    fn get_user_salt(&self, username: &String) -> String {
        /*
            Returns the salt of the username from the database, or "" if none
        */
        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT salt FROM {} WHERE username=?",
                self.users_table
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[Value::String(username.to_string()),])
            .unwrap();

        let salt: String;

        if let Some(row) = cursor.next().unwrap() {
            salt = row[0].as_string().unwrap().to_string();
        } else {
            salt = "".to_string();
        }

        salt
    }

    pub fn add_user(&self, username: &String, password: &String) -> i64 {
        /*
         * Inserts a new user
         */
        let salt = DbCrypto::gen_rand_salt();
        let password = DbCrypto::password_to_hash(&password, &salt);
        let guest_id = self.get_user_group_id_guest();

        let mut cursor = self.connection
            .prepare(format!(
                "INSERT INTO {} (username, password, salt, groupId) 
                                     VALUES (?, ?, ?, {});", // Hardcode group to guest for new users
                self.users_table, guest_id
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(username.to_string()),
                        Value::String(password.to_string()),
                        Value::String(salt.to_string())
                    ])
                    .unwrap();

        cursor.next().unwrap();

        self.get_last_insert_rowid()
    }

    pub fn is_user_login(&self, username: &String, password: &String) -> i64 {
        // Returns -1 on failed login, or user id on success

        let salt = self.get_user_salt(username);
        let password = DbCrypto::password_to_hash(&password, &salt);

        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT userId FROM {} WHERE username=? and password=?",
                self.users_table
            ))
            .unwrap()
            .into_cursor();

        cursor.bind(&[
                        Value::String(username.to_string()),
                        Value::String(password.to_string()),
                    ])
                    .unwrap();

        let user_id: i64;

        if let Some(row) = cursor.next().unwrap() {
            user_id = row[0].as_integer().unwrap_or(-1);
        } else {
            user_id = -1;
        }

        user_id
    }

    pub fn get_user_id(&self, username: &String) -> i64 {
        // Returns the user_id of the username if it exists or -1 if not
        
        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT userId FROM {} WHERE username=? ",
                self.users_table, 
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[Value::String(username.to_string()),])
            .unwrap();

        if let Some(row) = cursor.next().unwrap() {
            row[0].as_integer().unwrap_or(-1)
        } else {
            -1
        }
    }

    fn get_group_id_from_user_id(&self, user_id: i64) -> i64 {
        // Returns the user_id of the username if it exists or -1 if not
        
        let mut cursor = self
            .connection
            .prepare(format!(
                "SELECT groupId FROM {} WHERE userId=? ",
                self.users_table, 
            ))
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[Value::Integer(user_id),])
            .unwrap();

        if let Some(row) = cursor.next().unwrap() {
            row[0].as_integer().unwrap_or(-1)
        } else {
            -1
        }
    }

    pub fn is_user(&self, username: &String) -> bool {
        // Returns true if this username already exists in the database
        self.get_user_id(username) != -1
    }

    pub fn get_user_by_username(&self, username: &String) -> Option<UserInfo> {
        // Returns the user profile (if valid), else None

        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT userId, groupId FROM {} WHERE username=?;",
                self.users_table, 
            ),
            Some(&[Value::String(username.to_string()),])
        );

        if let Some(row) = cursor.next().unwrap() {
            let group_id = row[1].as_integer().unwrap();

            return Some(UserInfo {
                id:             row[0].as_integer().unwrap(),
                username:       username.to_string(),
                group:          self.get_user_group(group_id).unwrap(), //TODO: unwrap_or(group::default()) // aka guest
            });
        }
        None
    }

    pub fn get_user_by_id(&self, user_id: i64) -> Option<UserInfo> {
        // Returns the user profile (if valid), else None

        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT username, groupId FROM {} WHERE userId=?;",
                self.users_table, 
            ),
            Some(&[Value::Integer(user_id),])
        );

        if let Some(row) = cursor.next().unwrap() {
            let group_id = row[1].as_integer().unwrap();

            return Some(UserInfo {
                id:             user_id,
                username:       row[0].as_string().unwrap().to_string(),
                group:          self.get_user_group(group_id).unwrap(), //TODO: unwrap_or(group::default()) // aka guest
            });
        }
        None
    }

    pub fn get_all_guest_user_ids(&self) -> Vec<i64> {
        // Returns a vector of guest user ids

        let guest_id = self.get_user_group_id_guest();

        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT userId FROM {} WHERE groupId=?;",
                self.users_table, 
            ),
            Some(&[Value::Integer(guest_id),])
        );

        let mut user_ids = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            user_ids.push(row[0].as_integer().unwrap());
        }

        user_ids
    }

    pub fn get_all_users(&self) -> Vec<UserInfo> {
        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT userId, username, groupId FROM {};",
                self.users_table, 
            ),
            None
        );

        let mut users = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            let group_id = row[2].as_integer().unwrap();

            users.push(UserInfo {
                id:         row[0].as_integer().unwrap(),
                username:   row[1].as_string().unwrap().to_string(),
                group:      self.get_user_group(group_id).unwrap(), //TODO: unwrap_or(group::default()) // aka guest
            });
        }

        users
    }
}
