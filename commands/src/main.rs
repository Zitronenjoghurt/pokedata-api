use crate::cli::CLI;

mod cli;
mod commands;

fn main() -> Result<(), String> {
    let cli = CLI::build();
    cli.run()
}
