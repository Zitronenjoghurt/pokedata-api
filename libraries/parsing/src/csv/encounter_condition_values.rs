use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::encounter_condition_value::EncounterConditionValue;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValuesCSV {
    pub id: i32,
    pub encounter_condition_id: i32,
    pub identifier: String,
    pub is_default: i32,
}

impl CSVEntity for EncounterConditionValuesCSV {
    fn file_name() -> &'static str {
        "encounter_condition_values"
    }
}

impl ApiCSVEntity for EncounterConditionValuesCSV {
    type ApiType = EncounterConditionValue;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EncounterConditionValue {
            id: entry.id,
            encounter_condition_id: entry.encounter_condition_id,
            identifier: entry.identifier,
            is_default: entry.is_default == 1,
            names: data.get(entry.id),
        })
    }
}