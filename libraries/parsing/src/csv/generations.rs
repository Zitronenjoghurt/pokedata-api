use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::generation::Generation;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerationsCSV {
    pub id: i32,
    pub main_region_id: i32,
    pub identifier: String,
}

impl CSVEntity for GenerationsCSV {
    fn file_name() -> &'static str {
        "generations"
    }
}

impl ApiCSVEntity for GenerationsCSV {
    type ApiType = Generation;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            Generation {
                id: entry.id,
                main_region_id: entry.main_region_id,
                identifier: entry.identifier,
                names: data.get(entry.id),
            }
        )
    }
}