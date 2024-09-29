use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::{HashMapGroupById, IdValuePairingMapped};
use pokedata_api_entities::api::type_slots::TypeSlots;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonTypesCSV {
    pub pokemon_id: i32,
    pub type_id: i32,
    pub slot: i32,
}

impl CSVEntity for PokemonTypesCSV {
    fn file_name() -> &'static str {
        "pokemon_types"
    }
}

impl IdValuePairingMapped for PokemonTypesCSV {
    type Id = i32;
    type Key = i32;
    type Value = i32;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.pokemon_id, self.slot, self.type_id)
    }
}

impl PokemonTypesCSV {
    pub fn map(entries: Vec<PokemonTypesCSV>) -> HashMap<i32, TypeSlots> {
        let result = entries.group_by_id_mapped();
        result.into_iter()
            .map(|(pokemon_id, slots)| (pokemon_id, TypeSlots(slots)))
            .collect()
    }
}