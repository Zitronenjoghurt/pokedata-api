use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionGroupPokemonMoveMethodsCSV {
    pub version_group_id: u32,
    pub pokemon_move_method_id: u32,
}

impl CSVEntity for VersionGroupPokemonMoveMethodsCSV {
    fn file_name() -> &'static str {
        "version_group_pokemon_move_methods"
    }
}

impl IdValuePairing for VersionGroupPokemonMoveMethodsCSV {
    type Id = u32;
    type Value = u32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.version_group_id, self.pokemon_move_method_id)
    }
}