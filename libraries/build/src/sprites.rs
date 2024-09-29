use pokedata_api_entities::api::sprites::SpriteIndex;
use pokedata_api_utils::files::pokemon_sprites_index_file;
use std::fs;

pub fn load_sprite_index() -> SpriteIndex {
    let file_content = fs::read_to_string(pokemon_sprites_index_file())
        .expect("Failed to read sprite index file");
    serde_json::from_str(&file_content).expect("Failed to parse sprite index JSON")
}
