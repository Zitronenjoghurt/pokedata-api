use crate::entities::api::pokemon::Pokemon;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonCSV {
    pub id: u32,
    pub identifier: String,
    pub species_id: u32,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub base_experience: Option<u32>,
    pub order: Option<u32>,
    pub is_default: Option<u32>,
}

impl CSVEntity for PokemonCSV {
    fn file_name() -> &'static str {
        "pokemon"
    }
}

impl ApiCSVEntity for PokemonCSV {
    type ApiType = Pokemon;
    type ConversionData = ();

    fn convert(entry: Self, _data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            Pokemon {
                id: entry.id,
                identifier: entry.identifier,
                species_id: entry.species_id,
                height_decimeter: entry.height.unwrap_or_default(),
                weight_hectograms: entry.weight.unwrap_or_default(),
                base_experience: entry.base_experience.unwrap_or_default(),
                dex_order: entry.order.unwrap_or_default(),
                is_default: entry.is_default.unwrap_or_default() == 1,
            }
        )
    }
}