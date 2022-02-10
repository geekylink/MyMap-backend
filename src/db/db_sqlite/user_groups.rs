use sqlite::Value;
use serde::Serialize;

use crate::db::db_sqlite::DbSqlite;

#[derive(Serialize)]
pub struct UserGroupInfo {
    pub id:             i64,
    pub group_name:     String,
    pub permissions:    String,
}


const DEFAULT_GUEST_NAME: &str = "guest";
const DEFAULT_ADMIN_NAME: &str = "admin";

impl UserGroupInfo {
    /*pub fn default() -> UserGroupInfo {
        UserGroupInfo {
            id:             1
            group_name:     DEFAULT_GUEST_NAME.to_string(),
            permissions:    "".to_string(),
        }
    }*/
}

impl DbSqlite {
    pub fn init_user_groups(&self) {
        // Inits a guest & admin group
        if !self.is_group(DEFAULT_GUEST_NAME) {
            self.add_user_group(DEFAULT_GUEST_NAME, "");
        }
        if !self.is_group(DEFAULT_ADMIN_NAME) {
            self.add_user_group(DEFAULT_ADMIN_NAME, "*");
        }
    }

    pub fn get_user_group_id(&self, group_name: &str) -> i64 {
        // Gets group_id from group_name or -1 if none
        self.cursor_row_to_int(
            &mut self.prepare_sqlite_command(
                &format!(
                    "SELECT groupId FROM {} WHERE groupName=?;",
                    self.user_groups_table, 
                ),
                Some(&[Value::String(group_name.to_string()),])
            )
        )
        .unwrap_or(-1)
    }

    pub fn get_user_group_id_guest(&self) -> i64 {
        self.get_user_group_id(DEFAULT_GUEST_NAME)
    }

    pub fn get_user_group_id_admin(&self) -> i64 {
        self.get_user_group_id(DEFAULT_ADMIN_NAME)
    }

    pub fn add_user_group(&self, group_name: &str, permissions: &str) -> i64 {
        /*
         * Inserts a new user group
         */
        self.cursor_insert_and_get_row_id(
            &mut self.prepare_sqlite_command(
                &format!(
                    "INSERT INTO {} (groupName, permissions) 
                            VALUES  (?, ?);",
                    self.user_groups_table, 
                ),
                Some(&[
                    Value::String(group_name.to_string()),
                    Value::String(permissions.to_string()),
                ])
            )
        )
    }

    pub fn is_group(&self, group_name: &str) -> bool {
        // Returns true if this group already exists in the database
        self.cursor_has_row(
            &mut self.prepare_sqlite_command(
                &format!(
                    "SELECT groupId FROM {} WHERE groupName=?;",
                    self.user_groups_table, 
                ),
                Some(&[Value::String(group_name.to_string()),])
            )
        )
    }

    pub fn get_user_group(&self, group_id: i64) -> Option<UserGroupInfo> {
        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT groupName, permissions FROM {} WHERE groupId=?;",
                self.user_groups_table, 
            ),
            Some(&[Value::Integer(group_id),])
        );

        if let Some(row) = cursor.next().unwrap() {
            return Some(UserGroupInfo {
                id:             group_id,
                group_name:     row[0].as_string().unwrap().to_string(),
                permissions:    row[1].as_string().unwrap().to_string(),
            });
        }

        None
    }

    pub fn get_all_user_groups(&self) -> Vec<UserGroupInfo> {
        let mut cursor = self.prepare_sqlite_command(
            &format!(
                "SELECT groupId, groupName, permissions FROM {};",
                self.user_groups_table, 
            ),
            None
        );

        let mut groups = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            groups.push(UserGroupInfo {
                id:             row[0].as_integer().unwrap(),
                group_name:     row[1].as_string().unwrap().to_string(),
                permissions:    row[2].as_string().unwrap().to_string(),
            });
        }

        groups
    }

    pub fn get_group_permissions(&self, group_name: &str) -> String {
        self.cursor_row_to_string(
            &mut self.prepare_sqlite_command(
                &format!(
                    "SELECT permissions FROM {} WHERE groupName=?;",
                    self.user_groups_table, 
                ),
                Some(&[Value::String(group_name.to_string()),])
            )
        )
        .unwrap_or("".to_string())
    }
}