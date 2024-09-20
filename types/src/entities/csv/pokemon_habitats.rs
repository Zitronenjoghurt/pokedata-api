use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::api::pokemon_habitat::PokemonHabitat;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonHabitatsCSV {
    pub id: u32,
    pub identifier: String,
}

impl CSVEntity for PokemonHabitatsCSV {
    fn file_name() -> &'static str {
        "pokemon_habitats"
    }
}

impl ApiCSVEntity for PokemonHabitatsCSV {
    type ApiType = PokemonHabitat;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            PokemonHabitat {
                id: entry.id,
                identifier: entry.identifier,
                names: data.get(entry.id),
            }
        )
    }
}