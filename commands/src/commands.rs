use crate::cli::CliCommand;

pub mod hello;

pub fn get_commands() -> Vec<Box<dyn CliCommand>> {
    vec![
        Box::new(hello::HelloCommand)
    ]
}