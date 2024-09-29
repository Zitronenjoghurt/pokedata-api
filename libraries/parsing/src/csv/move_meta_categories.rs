use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_category::PokemonMoveCategory;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaCategoriesCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for MoveMetaCategoriesCSV {
    fn file_name() -> &'static str {
        "move_meta_categories"
    }
}

impl ApiCSVEntity for MoveMetaCategoriesCSV {
    type ApiType = PokemonMoveCategory;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(PokemonMoveCategory {
            id: entry.id,
            identifier: entry.identifier,
            descriptions: data.get(entry.id),
        })
    }
}