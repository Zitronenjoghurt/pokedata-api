use crate::csv::location_game_indices::{LocationGameIndicesCSV, LocationGenerationIdsCSV};
use crate::csv::location_names::{LocationNamesCSV, LocationSubtitlesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing::GroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::location::Location;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationsCSV {
    pub id: i32,
    pub region_id: Option<i32>,
    pub identifier: String,
}

impl CSVEntity for LocationsCSV {
    fn file_name() -> &'static str {
        "locations"
    }
}

impl ApiCSVEntity for LocationsCSV {
    type ApiType = Location;
    type ConversionData = LocationConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Location {
            id: entry.id,
            region_id: entry.region_id,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            subtitles: data.subtitles_map.get(entry.id),
            game_indices: data.game_indices_map.get(&entry.id).cloned().unwrap_or_default(),
            generation_ids: data.generation_ids_map.get(&entry.id).cloned().unwrap_or_default(),
            location_area_ids: vec![],
        })
    }
}

#[derive(Debug)]
pub struct LocationConversionData {
    pub names_map: LocalizedValuesMap,
    pub subtitles_map: LocalizedValuesMap,
    pub game_indices_map: HashMap<i32, Vec<i32>>,
    pub generation_ids_map: HashMap<i32, Vec<i32>>,
}

impl LocationConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: LocationNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            subtitles_map: LocationSubtitlesCSV::load(data_path).unwrap().into_localized_values_map(),
            game_indices_map: LocationGameIndicesCSV::load(data_path).unwrap().group_by_id(),
            generation_ids_map: LocationGenerationIdsCSV::load(data_path).unwrap().group_by_id(),
        }
    }
}