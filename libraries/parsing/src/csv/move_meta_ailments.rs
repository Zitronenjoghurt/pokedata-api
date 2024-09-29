use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_ailment::PokemonMoveAilment;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaAilmentsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for MoveMetaAilmentsCSV {
    fn file_name() -> &'static str {
        "move_meta_ailments"
    }
}

impl ApiCSVEntity for MoveMetaAilmentsCSV {
    type ApiType = PokemonMoveAilment;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(PokemonMoveAilment {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}