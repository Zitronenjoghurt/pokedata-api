use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::region::Region;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegionsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for RegionsCSV {
    fn file_name() -> &'static str {
        "regions"
    }
}

impl ApiCSVEntity for RegionsCSV {
    type ApiType = Region;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Region {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}