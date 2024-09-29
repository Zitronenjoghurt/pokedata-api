use crate::directories::{configuration_path, sprite_indices_path};
use std::path::PathBuf;

pub fn pokeapi_pokemon_sprites_index_config_file() -> PathBuf {
    configuration_path().join("pokeapi_pokemon_sprites_index.json")
}

pub fn pokemon_sprites_index_file() -> PathBuf {
    sprite_indices_path().join("pokemon.json")
}