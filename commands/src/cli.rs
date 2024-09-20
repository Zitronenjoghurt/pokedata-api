use crate::commands::get_commands;
use clap::{ArgMatches, Command};

pub trait CliCommand {
    fn name(&self) -> &str;
    fn command(&self) -> Command;
    fn run(&self, args: &ArgMatches) -> Result<(), String>;
}

#[derive(Default)]
pub struct CLI {
    commands: Vec<Box<dyn CliCommand>>,
}

impl CLI {
    pub fn build() -> Self {
        Self {
            commands: get_commands()
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let mut app = Command::new("Pokedata CLI")
            .version("1.0")
            .author("Zitronenjoghurt on GitHub")
            .about("Auto compiled CLI for handling various operational tasks for the Pokedata API.");

        for command in &self.commands {
            app = app.subcommand(command.command());
        }

        let matches = app.get_matches();

        for command in &self.commands {
            if let Some(matches) = matches.subcommand_matches(command.name()) {
                return command.run(matches);
            }
        }

        Err("No command was used, try ./cli --help".into())
    }
}