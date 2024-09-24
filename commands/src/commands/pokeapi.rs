use crate::cli::CliCommand;
use clap::{ArgMatches, Command};

mod build_sprites;
mod coverage;

const POKEAPI_OWNER: &'static str = "PokeAPI";
const POKEAPI_REPO: &'static str = "pokeapi";
const POKEAPI_SPRITES_REPO_HTTPS: &'static str = "https://github.com/PokeAPI/sprites.git";
const CSV_DATA_PATH: &'static str = "data/v2/csv";
const IGNORE_FILE_PREFIXES: &[&str] = &["conquest", "pokemon_species_prose", "super_contest", "type_game_indices"];

pub struct PokeApiCommand;

impl CliCommand for PokeApiCommand {
    fn name(&self) -> &str {
        "pokeapi"
    }

    fn command(&self) -> Command {
        Command::new("pokeapi")
            .about("All kinds of utility commands regarding PokeAPI")
            .subcommand(
                Command::new("coverage")
                    .about("Checks which of the PokeAPI CSV files aren't covered by this API yet.")
            )
            .subcommand(
                Command::new("build-sprites")
                    .about("Will download and build a sprite index with tokenized file names")
            )
    }

    fn run(&self, args: &ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            Some(("coverage", _)) => { coverage::coverage() }
            Some(("build-sprites", _)) => { build_sprites::build_sprites() }
            _ => {
                Err("Try using ./cli pokeapi --help'".to_string())
            }
        }
    }
}