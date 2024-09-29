use crate::csv::move_flag_prose::{MoveFlagDescriptionsCSV, MoveFlagNamesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_flag::PokemonMoveFlag;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveFlagsCSV {
    pub id: u32,
    pub identifier: String,
}

impl CSVEntity for MoveFlagsCSV {
    fn file_name() -> &'static str {
        "move_flags"
    }
}

impl ApiCSVEntity for MoveFlagsCSV {
    type ApiType = PokemonMoveFlag;
    type ConversionData = PokemonMoveFlagConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(PokemonMoveFlag {
            id: entry.id,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            descriptions: data.descriptions_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct PokemonMoveFlagConversionData {
    pub names_map: LocalizedValuesMap,
    pub descriptions_map: LocalizedValuesMap,
}

impl PokemonMoveFlagConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: MoveFlagNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            descriptions_map: MoveFlagDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}