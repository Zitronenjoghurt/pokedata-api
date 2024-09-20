use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::api::pokemon_shape::PokemonShape;
use crate::entities::csv::pokemon_shape_prose::{PokemonShapeAwesomeNamesCSV, PokemonShapeDescriptionsCSV, PokemonShapeNamesCSV};
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonShapesCSV {
    pub id: u32,
    pub identifier: String,
}

impl CSVEntity for PokemonShapesCSV {
    fn file_name() -> &'static str {
        "pokemon_shapes"
    }
}

impl ApiCSVEntity for PokemonShapesCSV {
    type ApiType = PokemonShape;
    type ConversionData = PokemonShapesConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            PokemonShape {
                id: entry.id,
                identifier: entry.identifier,
                names: data.names_map.get(entry.id),
                awesome_names: data.awesome_names_map.get(entry.id),
                descriptions: data.description_map.get(entry.id),
            }
        )
    }
}

#[derive(Default)]
pub struct PokemonShapesConversionData {
    pub names_map: LocalizedValuesMap,
    pub awesome_names_map: LocalizedValuesMap,
    pub description_map: LocalizedValuesMap,
}

impl PokemonShapesConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: PokemonShapeNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            awesome_names_map: PokemonShapeAwesomeNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            description_map: PokemonShapeDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}