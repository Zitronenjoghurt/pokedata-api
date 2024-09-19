use crate::entities::api::version::Version;
use crate::entities::csv::version_group_pokemon_move_methods::VersionGroupPokemonMoveMethodsCSV;
use crate::entities::csv::version_group_regions::VersionGroupRegionsCSV;
use crate::entities::csv::version_groups::VersionGroupsCSV;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_id::HasId;
use crate::entities::traits::id_value_pairing::GroupById;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct VersionGroup {
    pub id: u32,
    pub identifier: String,
    /// The generation this version group was introduced in.
    pub generation_id: u32,
    /// Order for sorting. Almost by date of release, except similar versions are grouped together.
    pub order: u32,
    /// The versions this version group owns.
    pub version_ids: Vec<u32>,
    /// A list of regions that can be visited in this version group.
    pub region_ids: Vec<u32>,
    /// A list of methods in which Pokémon can learn moves in this version group.
    pub move_method_ids: Vec<u32>,
}

impl HasId for VersionGroup {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_version_groups(
    version_groups_csv: Vec<VersionGroupsCSV>,
    versions_map: HashMap<u32, Version>,
    data_path: &PathBuf,
) -> Vec<VersionGroup> {
    let region_ids_map = VersionGroupRegionsCSV::load(data_path).unwrap().group_by_id();
    let move_methods_ids_map = VersionGroupPokemonMoveMethodsCSV::load(data_path).unwrap().group_by_id();

    let mut groups = Vec::with_capacity(version_groups_csv.len());

    for entry in version_groups_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let generation_id = match entry.generation_id {
            Some(generation_id) => generation_id,
            None => continue,
        };

        let group = VersionGroup {
            id,
            identifier,
            generation_id,
            order: entry.order.unwrap_or(0),
            version_ids: get_version_ids_by_versions(&versions_map, id),
            region_ids: region_ids_map.get(&id).unwrap_or(&Vec::new()).clone(),
            move_method_ids: move_methods_ids_map.get(&id).unwrap_or(&Vec::new()).clone(),
        };

        groups.push(group);
    }

    groups
}

fn get_version_ids_by_versions(version_map: &HashMap<u32, Version>, version_group_id: u32) -> Vec<u32> {
    version_map
        .values()
        .filter(|version| version.version_group_id == version_group_id)
        .map(|version| version.id)
        .collect()
}