use sqlite::Value;

use crate::db::crypto::DbCrypto;
use crate::db::db_sqlite::DbSqlite;

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

        let mut cursor = self.connection
            .prepare(format!(
                "INSERT INTO {} (username, password, salt) 
                                     VALUES (?, ?, ?);",
                self.users_table,
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

    pub fn is_user(&self, username: &String) -> bool {
        // Returns true if this username already exists in the database
        
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

        if let Some(_) = cursor.next().unwrap() {
            return true;
        }

        false
    }
}
