use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonVersionIdsCSV {
    pub pokemon_id: i32,
    pub version_id: i32,
}

impl CSVEntity for PokemonVersionIdsCSV {
    fn file_name() -> &'static str {
        "pokemon_game_indices"
    }
}

impl IdValuePairing for PokemonVersionIdsCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.pokemon_id, self.version_id)
    }
}