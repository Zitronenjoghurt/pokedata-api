use crate::commands::pokeapi::{POKEAPI_SPRITES_CONTENT_BASE_PATH, POKEAPI_SPRITES_REPO_HTTPS};
use git2::Repository;
use pokedata_api_utils::directories::data_path;
use pokedata_api_utils::files::pokeapi_pokemon_sprites_index_config_file;
use pokedata_api_utils::filesystem::{create_directory, get_file_name_without_extension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use uuid::Uuid;

/// HashMap<SpriteGroup, HashMap<SpriteType, Path>>
#[derive(Debug, Serialize, Deserialize)]
struct SpritePaths(HashMap<String, HashMap<String, String>>);

/// HashMap<PokemonId, HashMap<SpriteGroup, HashMap<SpriteType, FilePath>>>
/// PokemonId can include an optional form identifier appended with a hyphen, like: 493-bug for Arceus bug form
///
/// When self-host is enabled, the filepath will be the tokenized file names,
/// else they will be the path relative to the public PokeAPI sprites repository at:
/// https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/
/// for example:
/// "pokemon/1.png" points to https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/pokemon/1.png
#[derive(Debug, Serialize, Deserialize)]
struct SpriteIndex(HashMap<String, HashMap<String, HashMap<String, String>>>);

pub fn build_sprites(self_host: bool) -> Result<(), String> {
    let repo_path = data_path().join("pokemon-sprites-repo");
    if !repo_path.exists() {
        clone_repository(&repo_path)?;
    } else {
        println!("Found cloned sprites repository");
    }

    let self_host_path = data_path().join("pokemon-sprites-self-host");
    if self_host {
        create_directory(&self_host_path);
    }

    let pokemon_index = build_pokemon_sprites(&repo_path, &self_host_path, self_host);

    let output_path = output_json(
        serde_json::to_string_pretty(&pokemon_index).unwrap(),
        "pokemon",
    );

    println!("Done. Check {}", output_path.display());

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
) -> SpriteIndex
{
    let sprites_directory = repo_base_path.join("sprites").join(sub_folder_name);
    let public_hosting_path = PathBuf::from(POKEAPI_SPRITES_CONTENT_BASE_PATH).join(sub_folder_name);
    let mut sprite_index: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();

    for (sprite_group, type_path_map) in sprite_paths.0 {
        for (sprite_type, relative_path) in type_path_map {
            let directory_path = sprites_directory.join(&relative_path);
            if !directory_path.is_dir() {
                println!("{} is not a directory, skipping...", directory_path.display());
                continue;
            }

            let mut file_names: Vec<String> = Vec::new();
            for entry in read_dir(&directory_path)
                .expect(&format!("Failed to read directory {}", &directory_path.display())) {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if !file_path.is_file() {
                        continue;
                    }

                    let file_name = get_file_name_without_extension(&entry).unwrap();
                    if !file_name.starts_with(|c: char| c.is_ascii_digit()) {
                        continue;
                    }

                    file_names.push(file_name.clone());

                    let file_extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                    let final_path = determine_final_path(
                        &file_name,
                        file_extension,
                        &public_hosting_path,
                        &relative_path,
                        self_host,
                    );

                    sprite_index
                        .entry(file_name.clone())
                        .or_insert_with(HashMap::new)
                        .entry(sprite_group.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(sprite_type.clone(), final_path.clone());

                    if self_host {
                        println!("Path: {}", &file_path.display());
                        let destination = self_host_path.join(&final_path);
                        std::fs::create_dir_all(destination.parent().unwrap())
                            .expect("Failed to create directories");
                        std::fs::copy(&file_path, &destination)
                            .expect("Failed to copy file");
                    }
                }
            }
        }
    }

    SpriteIndex(sprite_index)
}

fn determine_final_path(
    file_name: &str,
    file_extension: &str,
    public_hosting_path: &PathBuf,
    relative_path: &str,
    self_host: bool,
) -> String {
    if self_host {
        format!("{}.{}", Uuid::new_v4().to_string(), file_extension)
    } else {
        let path = public_hosting_path.join(relative_path).join(format!("{}.{}", file_name, file_extension));
        path.to_string_lossy().replace('\\', "/")
    }
}

fn build_pokemon_sprites(repo_path: &PathBuf, self_host_path: &PathBuf, self_host: bool) -> SpriteIndex {
    let file = File::open(pokeapi_pokemon_sprites_index_config_file()).unwrap();
    let reader = BufReader::new(file);
    let pokemon_sprite_paths: SpritePaths = serde_json::from_reader(reader).unwrap();
    build_sprites_from_paths(repo_path, "pokemon", pokemon_sprite_paths, self_host_path, self_host)
}

fn output_json(
    json: String,
    file_name: &str,
) -> PathBuf {
    let output_path = data_path().join(format!("{}.json", file_name));
    let mut file = File::create(&output_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    output_path
}