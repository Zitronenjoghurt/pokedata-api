use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::encounter_method::EncounterMethod;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterMethodsCSV {
    pub id: i32,
    pub identifier: String,
    pub order: i32,
}

impl CSVEntity for EncounterMethodsCSV {
    fn file_name() -> &'static str {
        "encounter_methods"
    }
}

impl ApiCSVEntity for EncounterMethodsCSV {
    type ApiType = EncounterMethod;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EncounterMethod {
            id: entry.id,
            identifier: entry.identifier,
            order: entry.order,
            names: data.get(entry.id),
        })
    }
}