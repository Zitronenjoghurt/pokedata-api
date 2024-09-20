use crate::entities::api::version::Version;
use crate::entities::api::version_group::VersionGroup;
use crate::entities::csv::version_group_pokemon_move_methods::VersionGroupPokemonMoveMethodsCSV;
use crate::entities::csv::version_group_regions::VersionGroupRegionsCSV;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use crate::entities::traits::id_value_pairing::GroupById;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionGroupsCSV {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub order: u32,
}

impl CSVEntity for VersionGroupsCSV {
    fn file_name() -> &'static str {
        "version_groups"
    }
}

impl ApiCSVEntity for VersionGroupsCSV {
    type ApiType = VersionGroup;
    type ConversionData = VersionGroupConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(VersionGroup {
            id: entry.id,
            identifier: entry.identifier,
            generation_id: entry.generation_id,
            order: entry.order,
            version_ids: get_version_ids_by_versions(&data.versions_map, entry.id),
            region_ids: data.region_ids_map.get(&entry.id).cloned().unwrap_or_default(),
            move_method_ids: data.move_method_ids_map.get(&entry.id).cloned().unwrap_or_default(),
        })
    }
}

fn get_version_ids_by_versions(version_map: &HashMap<u32, Version>, version_group_id: u32) -> Vec<u32> {
    version_map
        .values()
        .filter(|version| version.version_group_id == version_group_id)
        .map(|version| version.id)
        .collect()
}

#[derive(Default)]
pub struct VersionGroupConversionData {
    pub versions_map: HashMap<u32, Version>,
    pub region_ids_map: HashMap<u32, Vec<u32>>,
    pub move_method_ids_map: HashMap<u32, Vec<u32>>,
}

impl VersionGroupConversionData {
    pub fn load(data_path: &PathBuf, versions_map: &HashMap<u32, Version>) -> Self {
        Self {
            versions_map: versions_map.clone(),
            region_ids_map: VersionGroupRegionsCSV::load(data_path).unwrap().group_by_id(),
            move_method_ids_map: VersionGroupPokemonMoveMethodsCSV::load(data_path).unwrap().group_by_id(),
        }
    }
}