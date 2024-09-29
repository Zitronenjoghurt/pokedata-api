use crate::csv::move_target_prose::{MoveTargetDescriptionsCSV, MoveTargetNamesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_target::PokemonMoveTarget;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveTargetsCSV {
    pub id: u32,
    pub identifier: String,
}

impl CSVEntity for MoveTargetsCSV {
    fn file_name() -> &'static str {
        "move_targets"
    }
}

impl ApiCSVEntity for MoveTargetsCSV {
    type ApiType = PokemonMoveTarget;
    type ConversionData = PokemonMoveTargetConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(PokemonMoveTarget {
            id: entry.id,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            descriptions: data.descriptions_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct PokemonMoveTargetConversionData {
    pub names_map: LocalizedValuesMap,
    pub descriptions_map: LocalizedValuesMap,
}

impl PokemonMoveTargetConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: MoveTargetNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            descriptions_map: MoveTargetDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}