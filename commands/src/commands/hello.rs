use crate::cli::CliCommand;
use clap::{Arg, ArgMatches, Command};

pub struct HelloCommand;

impl CliCommand for HelloCommand {
    fn name(&self) -> &str {
        "hello"
    }

    fn command(&self) -> Command {
        Command::new("hello")
            .about("Greets a person")
            .arg(Arg::new("name")
                .help("Name of the person to greet.")
                .required(true)
                .index(1))
    }

    fn run(&self, args: &ArgMatches) -> Result<(), String> {
        let name = args.get_one::<String>("name").unwrap();
        println!("Hello, {}!", name);
        Ok(())
    }
}