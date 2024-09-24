use crate::commands::pokeapi::{POKEAPI_SPRITES_REPO_HTTPS, POKEMON_SPRITES_INDEX_CONFIG_PATH};
use git2::Repository;
use pokedata_api_utils::filesystem::{create_directory, get_data_path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// HashMap<SpriteGroup, HashMap<SpriteType, Path>>
#[derive(Debug, Serialize, Deserialize)]
struct SpritePaths(HashMap<String, HashMap<String, String>>);

/// HashMap<PokemonId, HashMap<SpriteType, FilePath>>
/// PokemonId can include an optional form identifier appended with a hyphen, like: 493-bug for Arceus bug form
///
/// When self-host is enabled, the filepath will be the tokenized file names,
/// else they will be the path relative to the public PokeAPI sprites repository at:
/// https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/
/// for example:
/// "pokemon/1.png" points to https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/pokemon/1.png
#[derive(Debug, Serialize, Deserialize)]
struct SpriteIndex(HashMap<String, HashMap<String, String>>);

pub fn build_sprites(self_host: bool) -> Result<(), String> {
    let data_path = get_data_path().unwrap();
    let repo_path = data_path.join("sprites-repo");
    let self_host_path = data_path.join("sprites-self-host");

    if !repo_path.exists() {
        clone_repository(&repo_path)?;
    } else {
        println!("Found cloned sprites repository");
    }

    if self_host {
        create_directory(&self_host_path);
    }

    build_pokemon_sprites(&repo_path, &self_host_path, self_host);

    Ok(())
}

fn clone_repository(sprites_path: &PathBuf) -> Result<(), String> {
    create_directory(&sprites_path);
    println!("Cloning sprites repository... (this might take a bit)");
    match Repository::clone(POKEAPI_SPRITES_REPO_HTTPS, &sprites_path) {
        Ok(_) => println!("Successfully cloned sprites repository"),
        Err(e) => return Err(format!("Failed to clone sprites repository: {}", e))
    };

    Ok(())
}

fn build_sprites_from_paths(
    repo_base_path: &PathBuf,
    sub_folder_name: &str,
    sprite_paths: SpritePaths,
    self_host_path: &PathBuf,
    self_host: bool,
)
{
    let sprites_directory = repo_base_path.join("sprites").join(sub_folder_name);
}

fn build_pokemon_sprites(repo_path: &PathBuf, self_host_path: &PathBuf, self_host: bool) {
    let file = File::open(POKEMON_SPRITES_INDEX_CONFIG_PATH).unwrap();
    let reader = BufReader::new(file);
    let pokemon_sprite_paths: SpritePaths = serde_json::from_reader(reader).unwrap();
    build_sprites_from_paths(repo_path, "pokemon", pokemon_sprite_paths, self_host_path, self_host);
}