use crate::entities::api::generation::Generation;
use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerationsCSV {
    pub id: u32,
    pub main_region_id: u32,
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