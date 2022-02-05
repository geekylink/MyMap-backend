use crate::db::db_sqlite::DbSqlite;

impl DbSqlite {

    pub fn create_tables(&self) {
        // Create all tables
        self.create_files_table();
        self.create_locations_table();
        self.create_users_table();
    }

    fn create_files_table(&self) {
        /*
         * Create table for storing filenames and associated JSON data
         */

        self.connection
            .execute(format!("CREATE TABLE if not exists {} (
                                locationId INTEGER, 
                                filename TEXT, 
                                title TEXT,
                                description TEXT);",       self.files_table),)
            .unwrap();
    }

    fn create_locations_table(&self) {
        /*
         * Create table for storing location IDs & location data 
         */

        self.connection.execute(format!("CREATE TABLE if not exists {} (
                                locationId INTEGER PRIMARY KEY, 
                                label TEXT, 
                                lat INTEGER,
                                lon INTEGER,
                                data TEXT);",       self.locations_table),)
                        .unwrap();
    }

    fn create_users_table(&self) {
        self.connection.execute(format!("CREATE TABLE if not exists {} (
                                            userId INTEGER PRIMARY KEY, 
                                            username TEXT, 
                                            password TEXT,
                                            salt TEXT,
                                            sessionId TEXT);",
                                    self.users_table),
                        )
                        .unwrap();

    }


}
