use crate::csv::move_damage_class_prose::{MoveDamageClassDescriptionsCSV, MoveDamageClassNamesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_damage_class::PokemonMoveDamageClass;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveDamageClassesCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for MoveDamageClassesCSV {
    fn file_name() -> &'static str {
        "move_damage_classes"
    }
}

impl ApiCSVEntity for MoveDamageClassesCSV {
    type ApiType = PokemonMoveDamageClass;
    type ConversionData = PokemonMoveDamageClassConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(PokemonMoveDamageClass {
            id: entry.id,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            descriptions: data.descriptions_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct PokemonMoveDamageClassConversionData {
    pub names_map: LocalizedValuesMap,
    pub descriptions_map: LocalizedValuesMap,
}

impl PokemonMoveDamageClassConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: MoveDamageClassNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            descriptions_map: MoveDamageClassDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}