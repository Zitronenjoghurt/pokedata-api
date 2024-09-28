use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// HashMap<SpriteGroup, HashMap<SpriteType, Path>>
#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct SpritePaths(pub HashMap<String, HashMap<String, String>>);

/// HashMap<PokemonId, HashMap<SpriteGroup, HashMap<SpriteType, FilePath>>>
/// PokemonId can include an optional form identifier appended with a hyphen, like: 493-bug for Arceus bug form
///
/// When self-host is enabled, the filepath will be the tokenized file names,
/// else they will be the path relative to the public PokeAPI sprites repository at:
/// https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/
/// for example:
/// "pokemon/1.png" points to https://raw.githubusercontent.com/PokeAPI/sprites/refs/heads/master/sprites/pokemon/1.png
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteIndex(pub HashMap<String, HashMap<String, HashMap<String, String>>>);

impl SpriteIndex {
    /// Will return the sprite paths of the base form and potentially extra forms (if they exist)
    pub fn get_by_id(&self, id: u32) -> Option<(SpritePaths, HashMap<String, SpritePaths>)> {
        let base_id = id.to_string();
        let mut base_form = None;
        let mut additional_forms = HashMap::new();

        for (key, value) in self.0.iter() {
            if key == &base_id {
                base_form = Some(SpritePaths(value.clone()));
            } else if key.starts_with(&format!("{}-", base_id)) {
                let form = key.split('-').nth(1).unwrap_or("unknown").to_string();
                additional_forms.insert(form, SpritePaths(value.clone()));
            }
        }

        base_form.map(|base| (base, additional_forms))
    }
}