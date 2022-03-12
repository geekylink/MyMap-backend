// Note to self: must declare mods pub here even if not used here to be able to use in other files
pub mod cli;
pub mod db;
pub mod web_srv;

use crate::cli::CLICommands;

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let args = CLICommands::cli_arg_parse();
    CLICommands::cli_run(args.expect("Arguments could not be parsed.")).await;

    return Ok(());
}
