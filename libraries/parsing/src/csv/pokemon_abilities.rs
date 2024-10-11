use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use pokedata_api_entities::api::pokemon_ability::PokemonAbility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonAbilitiesCSV {
    pub pokemon_id: i32,
    pub ability_id: i32,
    pub is_hidden: i32,
    pub slot: i32,
}

impl CSVEntity for PokemonAbilitiesCSV {
    fn file_name() -> &'static str {
        "pokemon_abilities"
    }
}

impl IdValuePairing for PokemonAbilitiesCSV {
    type Id = i32;
    type Value = PokemonAbility;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.pokemon_id, PokemonAbility {
            ability_id: self.ability_id,
            is_hidden: self.is_hidden == 1,
            slot: self.slot,
        })
    }
}