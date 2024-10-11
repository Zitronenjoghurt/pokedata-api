use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::encounter_condition::EncounterCondition;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterConditionsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for EncounterConditionsCSV {
    fn file_name() -> &'static str {
        "encounter_conditions"
    }
}

impl ApiCSVEntity for EncounterConditionsCSV {
    type ApiType = EncounterCondition;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EncounterCondition {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
            condition_value_ids: vec![],
        })
    }
}