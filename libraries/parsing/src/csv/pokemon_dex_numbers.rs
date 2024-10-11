use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonDexNumbersCSV {
    pub species_id: i32,
    pub pokedex_id: i32,
    pub pokedex_number: i32,
}

impl CSVEntity for PokemonDexNumbersCSV {
    fn file_name() -> &'static str {
        "pokemon_dex_numbers"
    }
}

impl IdValuePairingMapped for PokemonDexNumbersCSV {
    type Id = i32;
    type Key = i32;
    type Value = i32;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.species_id, self.pokedex_id, self.pokedex_number)
    }
}