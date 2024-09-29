use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonColorNamesCSV {
    pub pokemon_color_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for PokemonColorNamesCSV {
    fn file_name() -> &'static str {
        "pokemon_color_names"
    }
}

impl HasLocalizedValues for PokemonColorNamesCSV {
    fn id(&self) -> i32 {
        self.pokemon_color_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}