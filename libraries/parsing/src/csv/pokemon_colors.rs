use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_color::PokemonColor;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonColorsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for PokemonColorsCSV {
    fn file_name() -> &'static str {
        "pokemon_colors"
    }
}

impl ApiCSVEntity for PokemonColorsCSV {
    type ApiType = PokemonColor;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            PokemonColor {
                id: entry.id,
                identifier: entry.identifier,
                names: data.get(entry.id),
            }
        )
    }
}