use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_type::PokemonType;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypesCSV {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub damage_class_id: Option<u32>,
    pub is_major_type: u32,
}

impl CSVEntity for TypesCSV {
    fn file_name() -> &'static str {
        "types"
    }

    fn base_download_url() -> &'static str {
        "https://raw.githubusercontent.com/Zitronenjoghurt/pokeapi-data-fix/master/data/v2/csv/"
    }
}

impl ApiCSVEntity for TypesCSV {
    type ApiType = PokemonType;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            PokemonType {
                id: entry.id,
                identifier: entry.identifier,
                generation_id: entry.generation_id,
                is_major_type: entry.is_major_type == 1,
                damage_class_id: entry.damage_class_id,
                names: data.get(entry.id),
            }
        )
    }
}