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
            Arg::new("test")
                .short('t')
                .long("test")
                .takes_value(false)
                .help("Runs a test"),
        )
        .get_matches();

    args
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

    if args.is_present("test") {
        println!("running test");

        let db = db::new();
        let res = db.get_location_id(&String::from("lolland"), -69.0, 420.0);

        println!("res: {}", res);
        return Ok(());
    } else if args.is_present("add-user") {
        let mut input = String::new();

        let username = args.value_of("add-user").unwrap().to_string();
        let db = db::new();

        if db.is_user(&username) {
            println!("This user already exists!");
            return Ok(());
        }

        println!("Enter your password:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read username");
        print!("\x1B[2J\x1B[1;1H");
        println!("User added");
        return Ok(());
    } else if args.is_present("no-auth-api") {
        server.disable_auth_api();
    }

    server.launch_server().await
}
