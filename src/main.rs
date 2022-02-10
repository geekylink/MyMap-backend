use clap::Arg;
use std::io;

// Note to self: must declare mods pub here even if not used here to be able to use in other files
pub mod db;
pub mod web_srv;

fn cli_arg_parse() -> clap::ArgMatches {
    // Parse and return CLI arguments
    // TODO: add argument validation
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
            Arg::new("add-user")
                .long("add-user")
                .takes_value(true)
                .help("Adds a user to the database."),
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
                .short('t')
                .long("test")
                .takes_value(false)
                .help("Runs a test"),
        )
        .get_matches();

    args
}

fn cli_add_user_to_db(username: String) -> bool {
    // Adds username to db, asks for password from CLI
    // TODO: improve password input somehow?
    let password: String;
    let db = db::new();

    if db.is_user(&username) {
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
    db.add_user(&username, &password);
    println!("User added: {}", &username);

    true
}

fn list_guests() {
    // Prints out all guest accounts so we can decide to accept/delete
    let db = db::new();
    let guest_ids = db.get_all_guest_user_ids();

    println!("All guest accounts:");
    for i in 0..guest_ids.len() {
        let user_id = guest_ids[i];
        let user = db.get_user_by_id(user_id).unwrap();
        println!("{}: {}", user_id, user.username);
    }
}

fn list_all_users() {
    let db = db::new();
    let users = db.get_all_users();

    println!("All users:");
    for i in 0..users.len() {
        println!("{}: {} ({}: Permissions: '{}')", 
            users[i].id, 
            users[i].username, 
            users[i].group.group_name,
            users[i].group.permissions);
    }
}

fn list_all_groups() {
    let db = db::new();
    let groups = db.get_all_user_groups();

    println!("All groups:");
    for i in 0..groups.len() {
            println!("{}: Permissions: '{}'", groups[i].group_name, groups[i].permissions);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = cli_arg_parse();

    let address = args.value_of("address").unwrap_or("");
    let port = args
        .value_of("port")
        .unwrap_or("")
        .parse::<i32>()
        .unwrap_or(-1);

    let mut server = web_srv::APIServer::new(&address, port);

    if args.is_present("add-user") {
        cli_add_user_to_db(args.value_of("add-user").unwrap().to_string());
        return Ok(());
    } 
    else if args.is_present("no-auth-api") {
        server.disable_auth_api();
    }
    else if args.is_present("list-guests") {
        list_guests();
        return Ok(());
    }
    else if args.is_present("list-users") {
        list_all_users();
        return Ok(());
    }
    else if args.is_present("list-groups") {
        list_all_groups();
        return Ok(());
    }
    else if args.is_present("test") {
        let db = db::new();
        return Ok(());
    }

    server.launch_server().await
}
