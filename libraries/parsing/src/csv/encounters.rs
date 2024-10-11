use crate::csv::encounter_condition_value_map::EncounterConditionValueMapCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing::GroupById;
use pokedata_api_entities::api::encounter::Encounter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncountersCSV {
    pub id: i32,
    pub version_id: i32,
    pub location_area_id: i32,
    pub encounter_slot_id: i32,
    pub pokemon_id: i32,
    pub min_level: i32,
    pub max_level: i32,
}

impl CSVEntity for EncountersCSV {
    fn file_name() -> &'static str {
        "encounters"
    }
}

impl ApiCSVEntity for EncountersCSV {
    type ApiType = Encounter;
    type ConversionData = EncounterConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Encounter {
            id: entry.id,
            version_id: entry.version_id,
            location_area_id: entry.location_area_id,
            encounter_slot_id: entry.encounter_slot_id,
            pokemon_id: entry.pokemon_id,
            min_level: entry.min_level,
            max_level: entry.max_level,
            encounter_condition_value_ids: data.encounter_condition_value_map.get(&entry.id).cloned().unwrap_or_default(),
        })
    }
}

#[derive(Debug, Default)]
pub struct EncounterConversionData {
    pub encounter_condition_value_map: HashMap<i32, Vec<i32>>,
}

impl EncounterConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            encounter_condition_value_map: EncounterConditionValueMapCSV::load(data_path).unwrap().group_by_id()
        }
    }
}
