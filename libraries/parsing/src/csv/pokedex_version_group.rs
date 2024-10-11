use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokedexVersionGroupsCSV {
    pub pokedex_id: i32,
    pub version_group_id: i32,
}

impl CSVEntity for PokedexVersionGroupsCSV {
    fn file_name() -> &'static str {
        "pokedex_version_groups"
    }
}

impl IdValuePairing for PokedexVersionGroupsCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.pokedex_id, self.version_group_id)
    }
}