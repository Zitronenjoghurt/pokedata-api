use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::evolution_trigger::EvolutionTrigger;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EvolutionTriggersCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for EvolutionTriggersCSV {
    fn file_name() -> &'static str {
        "evolution_triggers"
    }
}

impl ApiCSVEntity for EvolutionTriggersCSV {
    type ApiType = EvolutionTrigger;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EvolutionTrigger {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}