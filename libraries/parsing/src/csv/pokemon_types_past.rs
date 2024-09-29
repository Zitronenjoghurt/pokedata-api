use crate::csv_entity::CSVEntity;
use pokedata_api_entities::api::type_slots::{TypeSlots, TypeSlotsPast};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonTypesPastCSV {
    pub pokemon_id: i32,
    pub generation_id: i32,
    pub type_id: i32,
    pub slot: i32,
}

impl CSVEntity for PokemonTypesPastCSV {
    fn file_name() -> &'static str {
        "pokemon_types_past"
    }
}

impl PokemonTypesPastCSV {
    /// Creates a mapped representation of the past types of a pokemon by generation:
    /// HashMap<pokemon_id, HashMap<generation_id, HashMap<slot, type_id>>>
    pub fn map(entries: Vec<PokemonTypesPastCSV>) -> HashMap<i32, TypeSlotsPast> {
        let mut result: HashMap<i32, TypeSlotsPast> = HashMap::new();

        for entry in entries {
            result
                .entry(entry.pokemon_id)
                .or_insert_with(TypeSlotsPast::default)
                .0
                .entry(entry.generation_id)
                .or_insert_with(TypeSlots::default)
                .0
                .insert(entry.slot, entry.type_id);
        }

        result
    }
}