use std::io;
use clap::Arg;

use crate::db::MapDB;
use crate::web_srv::APIServer;

pub struct CLICommands {}

impl CLICommands {
    pub fn cli_arg_parse() -> Option<clap::ArgMatches> {
        // Parse and return CLI arguments

        let args = clap::App::new("Rust Server Demo")
            .version("0.0.1")
            .author("James Danielson")
            .about("Run server as ")
            .arg(
                Arg::new("address")
                    .short('a')
                    .long("addr")
                    .takes_value(true)
                    .help("Address to listen on (Default: 0.0.0.0)"),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .takes_value(true)
                    .help("Address to listen on (Default: 8080)"),
            )
            .arg(
                Arg::new("no-auth-api")
                    .long("no-auth-api")
                    .takes_value(false)
                    .help("Disables all authentication and all write access."),
            )
            .arg(
                Arg::new("add-group")
                    .long("add-group")
                    .takes_value(false)
                    .help("Adds a group to the database. Requires: --group --permissions."),
            )
            .arg(
                Arg::new("edit-group")
                    .long("edit-group")
                    .takes_value(false)
                    .help("Edits a group's permissions in the database. Requires: --group --permissions."),
            )
            .arg(
                Arg::new("add-user")
                    .long("add-user")
                    .takes_value(false)
                    .help("Adds a user to the database."),
            )
            .arg(
                Arg::new("add-to-group")
                    .long("add-to-group")
                    .takes_value(false)
                    .help("Adds a user to a group. Requires: --user --group"),
            )
            .arg(
                Arg::new("list-guests")
                    .long("list-guests")
                    .takes_value(false)
                    .help("Lists all guest accounts in the database"),
            )
            .arg(
                Arg::new("list-users")
                    .long("list-users")
                    .takes_value(false)
                    .help("Lists all users in the database"),
            )
            .arg(
                Arg::new("list-groups")
                    .long("list-groups")
                    .takes_value(false)
                    .help("Lists all groups in the database"),
            )
            .arg(
                Arg::new("test")
                    .long("test")
                    .takes_value(false)
                    .help("Does something"),
            )
            .arg(
                Arg::new("group")
                    .long("group")
                    .takes_value(true)
                    .help("For specifying which group to perform an action on"),
            )
            .arg(
                Arg::new("user")
                    .long("user")
                    .takes_value(true)
                    .help("For specifying which user to perform an action on"),
            )
            .arg(
                Arg::new("permissions")
                    .long("permissions")
                    .takes_value(true)
                    .help("Permissions to attach to a group"),
            )
            .get_matches();
    
        // Argument validation
        if args.is_present("add-to-group") && (!args.is_present("user") || !args.is_present("group")) {
            println!("Error: Must specify --user <USER> & --group <GROUP>");
        }
        else if args.is_present("add-group") && (!args.is_present("group") || !args.is_present("permissions")) {
            println!("Error: Must specify --permissions <permissions> & --group <GROUP>");
        }
        else if args.is_present("edit-group") && (!args.is_present("group") || !args.is_present("permissions")) {
            println!("Error: Must specify --permissions <permissions> & --group <GROUP>");
        }
        else if args.is_present("add-user") && !args.is_present("user") {
            println!("Error: Must specify --user <USER> to add");
        }
        else {
            return Some(args); // Good arguments
        }
    
        None
    }

    pub async fn cli_run(args: clap::ArgMatches) {
        let address = args.value_of("address").unwrap_or("");
        let port = args
            .value_of("port")
            .unwrap_or("")
            .parse::<i32>()
            .unwrap_or(-1);

        // Various CLI commands to run instead of the server
        if args.is_present("add-user") && args.is_present("user") {
            CLICommands::add_user_to_db(args.value_of("user").unwrap().to_string()).await;
        }
        else if args.is_present("list-guests") {
            CLICommands::list_guests().await;
        }
        else if args.is_present("list-users") {
            CLICommands::list_all_users().await;
        }
        else if args.is_present("list-groups") {
            CLICommands::list_all_groups().await;
        }
        else if args.is_present("add-to-group") && args.is_present("user") && args.is_present("group") {
            CLICommands::add_user_to_group(args.value_of("user").unwrap(), args.value_of("group").unwrap()).await;
        }
        else if args.is_present("add-group") && args.is_present("group") && args.is_present("permissions") {
            CLICommands::add_user_group(args.value_of("group").unwrap(), args.value_of("permissions").unwrap()).await;
        }
        else if args.is_present("edit-group") && args.is_present("group") && args.is_present("permissions") {
            CLICommands::edit_user_group(args.value_of("group").unwrap(), args.value_of("permissions").unwrap()).await;
        }
        else if args.is_present("test") {

        }
        else {
            let mut server = APIServer::new(&address, port).await;

            if args.is_present("no-auth-api") {
                server.disable_auth_api();
            }

            server.launch_server().await.ok();
        }
    }

    async fn add_user_to_db(username: String) -> bool {
        // Adds username to db, asks for password from CLI
        // TODO: improve password input somehow?

        let password: String;
        let db = MapDB::new().await;

        if db.is_user(&username).await {
            println!("User '{}' already exists!", &username);
            return false
        } 

        println!("Enter your password:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read username");
        password = input.trim().to_string();

        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("Adding...");
        db.add_user(&username, &password).await;
        println!("User added: {}", &username);

        true
    }

    async fn list_all_users() {
        // List all users in the database
        let db = MapDB::new().await;
        let users = db.get_all_users().await;
    
        println!("All users:");
        for i in 0..users.len() {
            println!("{}, ({} permissions: '{}')", 
                //users[i].id, 
                users[i].username, 
                users[i].group.group_name,
                users[i].group.permissions
            );
        }
    }


    async fn list_all_groups() {
        let db = MapDB::new().await;
        let groups = db.get_all_user_groups().await;

        println!("All groups:");
        for i in 0..groups.len() {
                println!("{}: Permissions: '{}'", groups[i].group_name, groups[i].permissions);
        }
    }

    async fn list_guests() {
        let db = MapDB::new().await;
        let users = db.get_all_users().await;
    
        println!("All guest users:");
        for i in 0..users.len() {
            if users[i].group.group_name.eq("guest") {
                println!("{}, ({} permissions: '{}')", 
                    users[i].username, 
                    users[i].group.group_name,
                    users[i].group.permissions
                );
            }
        }
    }

    async fn add_user_to_group(username: &str, group_name: &str) {
        let db = MapDB::new().await;
    
        if !db.is_user(&username).await {
            println!("Invalid username");
            return;
        }
    
        if !db.is_user_group(&group_name).await {
            println!("Invalid group");
            return;
        }
    
        let user_id  = db.get_user_id(&username).await;
        let group = db.get_user_group_by_name(&group_name).await.unwrap();
    
        db.add_user_to_group(user_id, group.id).await;
        println!("User '{}' added to group '{}'", username, group_name);
    }
    
    async fn add_user_group(group_name: &str, permissions: &str) {
        let db = MapDB::new().await;
    
        if db.is_user_group(&group_name).await {
            println!("'{}' is already a group!", group_name);
            return;
        }
    
        db.add_user_group(group_name, permissions).await;
        println!("Added group '{}'", group_name);
    }
    
    async fn edit_user_group(group_name: &str, permissions: &str) {
        let db = MapDB::new().await;
    
        if !db.is_user_group(&group_name).await {
            println!("'{}' must already be a group to edit it.", group_name);
            return;
        }
    
        db.edit_user_group(group_name, permissions).await;
        println!("Edited group '{}', new permissions: {}", group_name, permissions);
    }
}