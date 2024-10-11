use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use pokedata_api_entities::api::pokemon_ability::PokemonAbility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonAbilitiesPastCSV {
    pub pokemon_id: i32,
    pub generation_id: i32,
    pub ability_id: i32,
    pub is_hidden: i32,
    pub slot: i32,
}

impl CSVEntity for PokemonAbilitiesPastCSV {
    fn file_name() -> &'static str {
        "pokemon_abilities_past"
    }
}

impl IdValuePairingMapped for PokemonAbilitiesPastCSV {
    type Id = i32;
    type Key = i32;
    type Value = PokemonAbility;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.pokemon_id, self.generation_id, PokemonAbility {
            ability_id: self.ability_id,
            is_hidden: self.is_hidden == 1,
            slot: self.slot,
        })
    }
}