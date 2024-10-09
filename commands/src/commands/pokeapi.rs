use crate::cli::CliCommand;
use clap::{Arg, ArgMatches, Command};

mod build_sprites;
mod coverage;

const CSV_DATA_PATH: &str = "data/v2/csv";
const IGNORE_FILE_PREFIXES: &[&str] = &["conquest", "item_flavor_summaries", "move_flavor_summaries", "pokemon_species_flavor_summaries", "pokemon_species_prose", "super_contest", "type_game_indices"];
const POKEAPI_OWNER: &str = "PokeAPI";
const POKEAPI_REPO: &str = "pokeapi";
const POKEAPI_SPRITES_REPO_HTTPS: &str = "https://github.com/PokeAPI/sprites.git";
const POKEAPI_SPRITES_CONTENT_BASE_PATH: &str = "https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/";

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
                    .about("Will download and build a sprite index")
                    .arg(
                        Arg::new("sh")
                            .long("self-host")
                            .help("Enable self-hosting mode. This will tokenize the file names and copy them to a separate section of the data directory (see documentation).")
                            .action(clap::ArgAction::SetTrue)
                    )
            )
    }

    fn run(&self, args: &ArgMatches) -> Result<(), String> {
        match args.subcommand() {
            Some(("coverage", _)) => { coverage::coverage() }
            Some(("build-sprites", sub_matches)) => {
                let self_host = sub_matches.get_flag("sh");
                build_sprites::build_sprites(self_host)
            }
            _ => {
                Err("Try using ./cli pokeapi --help'".to_string())
            }
        }
    }
}