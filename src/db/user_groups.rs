use serde::Serialize;

use crate::db::MapDB;

const DEFAULT_GUEST_NAME: &str = "guest";
const DEFAULT_ADMIN_NAME: &str = "admin";

#[derive(Serialize)]
pub struct UserGroupInfo {
    pub id:             i64,
    pub group_name:     String,
    pub permissions:    String,
}

impl MapDB {
    pub async fn add_user_group(&self, group_name: &str, permissions: &str) -> i64 {
        sqlx::query("INSERT INTO user_groups 
                                    (group_name, permissions) 
                            VALUES  (?, ?);")
                .bind(&group_name)
                .bind(&permissions)
                .execute(&self.pool)
                .await
                .expect("Inserting new user group into db")
                .last_insert_rowid()
    }

    pub async fn edit_user_group(&self, group_name: &str, permissions: &str) {
        sqlx::query("UPDATE user_groups 
                            SET permissions=?
                            WHERE group_name=?")
                .bind(&permissions)
                .bind(&group_name)
                .execute(&self.pool)
                .await
                .expect("Updating user group in db");
    }

    pub async fn get_user_group_by_id(&self, group_id: i64) -> Option<UserGroupInfo> {
        sqlx::query_as!(UserGroupInfo, "SELECT *
                                        FROM user_groups 
                                        WHERE id=?;", 
                                   group_id)
                    .fetch_one(&self.pool)
                    .await.ok()
    }

    pub async fn get_all_user_groups(&self) -> Vec<UserGroupInfo> {
        sqlx::query_as!(UserGroupInfo, 
                    "SELECT * FROM user_groups")
                .fetch_all(&self.pool)
                .await.ok().unwrap()
    }

    pub async fn get_user_group_by_name(&self, group_name: &str) -> Option<UserGroupInfo> {
        sqlx::query_as!(UserGroupInfo, "SELECT *
                                        FROM user_groups 
                                        WHERE group_name=?;", 
                                   group_name)
                    .fetch_one(&self.pool)
                    .await.ok()
    }

    // Returns group_id from group_name
    pub async fn get_user_group_id(&self, group_name: &str) -> i64 {
        let row: (i64,) = sqlx::query_as("SELECT id 
                        FROM user_groups 
                        WHERE group_name=?;")
                .bind(&group_name)
                .fetch_one(&self.pool)
                .await.ok().expect(&format!("Could not query user_group '{}'", group_name));

        row.0
    }

    pub async fn get_user_group_id_guest(&self) -> i64 {
        self.get_user_group_id(DEFAULT_GUEST_NAME).await
    }

    pub async fn get_user_group_id_admin(&self) -> i64 {
        self.get_user_group_id(DEFAULT_ADMIN_NAME).await
    }

    pub async fn is_user_group(&self, group_name: &str) -> bool {
        !self.get_user_group_by_name(group_name).await.is_none()
    }
}
