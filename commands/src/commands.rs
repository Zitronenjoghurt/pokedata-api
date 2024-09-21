use crate::cli::CliCommand;

mod hello;
mod pokeapi;

pub fn get_commands() -> Vec<Box<dyn CliCommand>> {
    vec![
        Box::new(hello::HelloCommand),
        Box::new(pokeapi::PokeApiCommand)
    ]
}