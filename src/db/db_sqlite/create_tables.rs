use crate::db::db_sqlite::DbSqlite;

impl DbSqlite {
    pub fn create_tables(&self) {
        // Create all tables

        self.create_files_table();
        self.create_locations_table();
        self.create_users_table();
        self.create_user_groups_table();
        self.create_comments_table();
    }

    fn create_files_table(&self) {
        /*
         * Create table for storing filenames and associated JSON data
         */

        self.connection
            .execute(format!(
                "CREATE TABLE if not exists {} (
                                fileId INTEGER PRIMARY KEY,
                                locationId INTEGER, 
                                filename TEXT, 
                                title TEXT,
                                description TEXT,
                                owner INTEGER);",
                self.files_table
            ))
            .unwrap();
    }

    fn create_locations_table(&self) {
        /*
         * Create table for storing location IDs & location data
         */

        self.connection
            .execute(format!(
                "CREATE TABLE if not exists {} (
                                locationId INTEGER PRIMARY KEY, 
                                label TEXT, 
                                lat REAL, 
                                lon REAL,
                                type TEXT,
                                owner INTEGER);", // REAL or INTEGER ? for lat/lon
                self.locations_table
            ))
            .unwrap();
    }

    fn create_users_table(&self) {

        self.connection
            .execute(format!(
                "CREATE TABLE if not exists {} (
                                            userId INTEGER PRIMARY KEY, 
                                            username TEXT, 
                                            password TEXT,
                                            salt TEXT,
                                            groupId Integer);",
                self.users_table
            ))
            .unwrap();
    }

    fn create_user_groups_table(&self) {

        self.connection
            .execute(format!(
                "CREATE TABLE if not exists {} (
                                            groupId INTEGER PRIMARY KEY, 
                                            groupName TEXT, 
                                            permissions TEXT);",
                self.user_groups_table
            ))
            .unwrap();
    }

    fn create_comments_table(&self) {

        self.connection
            .execute(format!(
                "CREATE TABLE if not exists {} (
                                            commentId INTEGER PRIMARY KEY, 
                                            locationId INTEGER, 
                                            fileId INTEGER, 
                                            owner INTEGER,
                                            postedDate REAL,
                                            lastEditDate REAL);", // Can comment on location or file
                self.user_groups_table
            ))
            .unwrap();
    }
}
