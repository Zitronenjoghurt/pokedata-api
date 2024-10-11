use crate::csv::pokedex_prose::{PokedexDescriptionsCSV, PokedexNamesCSV};
use crate::csv::pokedex_version_group::PokedexVersionGroupsCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing::GroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokedex::Pokedex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokedexesCSV {
    pub id: i32,
    pub region_id: Option<i32>,
    pub identifier: String,
    pub is_main_series: i32,
}

impl CSVEntity for PokedexesCSV {
    fn file_name() -> &'static str {
        "pokedexes"
    }
}

impl ApiCSVEntity for PokedexesCSV {
    type ApiType = Pokedex;
    type ConversionData = PokedexConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Pokedex {
            id: entry.id,
            region_id: entry.region_id,
            identifier: entry.identifier,
            is_main_series: entry.is_main_series == 1,
            version_group_ids: data.version_group_ids_map.get(&entry.id).cloned().unwrap_or_default(),
            names: data.names_map.get(entry.id),
            descriptions: data.descriptions_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct PokedexConversionData {
    pub version_group_ids_map: HashMap<i32, Vec<i32>>,
    pub names_map: LocalizedValuesMap,
    pub descriptions_map: LocalizedValuesMap,
}

impl PokedexConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            version_group_ids_map: PokedexVersionGroupsCSV::load(data_path).unwrap().group_by_id(),
            names_map: PokedexNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            descriptions_map: PokedexDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}