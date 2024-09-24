use crate::cli::CLI;
use dotenv::dotenv;

mod cli;
mod commands;

fn main() -> Result<(), String> {
    dotenv().ok();

    let cli = CLI::build();
    cli.run()
}