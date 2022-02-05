use sqlite::State;

use crate::db::crypto::DbCrypto;
use crate::db::db_sqlite::DbSqlite;

impl DbSqlite {
    fn get_user_salt(&self, username: &String) -> String {
        /*
            Returns the salt of the username from the database, or "" if none
        */
        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT salt FROM {} WHERE username='{}'",
                self.users_table, username
            ))
            .unwrap();
        let salt: String;

        if let State::Row = statement.next().unwrap() {
            salt = statement.read::<String>(0).unwrap_or("".to_string());
        } else {
            salt = "".to_string();
        }

        salt
    }

    pub fn get_last_insert_rowid(&self) -> i64 {
        let mut statement = self
            .connection
            .prepare("select last_insert_rowid();")
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            return statement.read::<i64>(0).unwrap();
        }

        -1
    }

    pub fn add_user(&self, username: &String, password: &String) -> i64 {
        /*
         * Inserts a new user
         */
        let salt = DbCrypto::gen_rand_salt();
        let password = DbCrypto::password_to_hash(&password, &salt);

        self.connection
            .execute(format!(
                "INSERT INTO {} (username, password, salt) 
                                     VALUES ('{}', '{}', '{}');",
                self.users_table, username, password, salt
            ))
            .unwrap();

        self.get_last_insert_rowid()
    }

    pub fn is_user_login(&self, username: &String, password: &String) -> i64 {
        // Returns -1 on failed login, or user id on success

        let salt = self.get_user_salt(username);
        let password = DbCrypto::password_to_hash(&password, &salt);

        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT userId FROM {} WHERE username='{}' and password='{}'",
                self.users_table, username, password
            ))
            .unwrap();

        let user_id: i64;

        if let State::Row = statement.next().unwrap() {
            user_id = statement.read::<i64>(0).unwrap();
        } else {
            user_id = -1;
        }

        user_id
    }

    pub fn is_user(&self, username: &String) -> bool {
        let mut statement = self
            .connection
            .prepare(format!(
                "SELECT userId FROM {} WHERE username='{}' ",
                self.users_table, username
            ))
            .unwrap();

        if let State::Row = statement.next().unwrap() {
            return true;
        }

        false
    }
}
