use crate::csv::location_area_encounter_rates::LocationAreaEncounterRatesCSV;
use crate::csv::location_area_prose::LocationAreaProseCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing_mapped::HashMapGroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::location_area::LocationArea;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationAreasCSV {
    pub id: i32,
    pub location_id: i32,
    pub game_index: i32,
    pub identifier: Option<String>,
}

impl CSVEntity for LocationAreasCSV {
    fn file_name() -> &'static str {
        "location_areas"
    }
}

impl ApiCSVEntity for LocationAreasCSV {
    type ApiType = LocationArea;
    type ConversionData = LocationAreaConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(LocationArea {
            id: entry.id,
            location_id: entry.location_id,
            game_index: entry.game_index,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            encounter_rates: data.encounter_rates_map.get(&entry.id).cloned().unwrap_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct LocationAreaConversionData {
    pub names_map: LocalizedValuesMap,
    pub encounter_rates_map: HashMap<i32, HashMap<i32, (i32, i32)>>,
}

impl LocationAreaConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: LocationAreaProseCSV::load(data_path).unwrap().into_localized_values_map(),
            encounter_rates_map: LocationAreaEncounterRatesCSV::load(data_path).unwrap().group_by_id_mapped(),
        }
    }
}