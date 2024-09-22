use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonVersionIdsCSV {
    pub pokemon_id: u32,
    pub version_id: u32,
}

impl CSVEntity for PokemonVersionIdsCSV {
    fn file_name() -> &'static str {
        "pokemon_game_indices"
    }
}

impl IdValuePairing for PokemonVersionIdsCSV {
    type Id = u32;
    type Value = u32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.pokemon_id, self.version_id)
    }
}